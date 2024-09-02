use {
    crate::{
        client::ApiClient,
        constants::{WS_MARKET_DATA_FEED_AUTHORIZE_ENDPOINT, WS_PORTFOLIO_FEED_AUTHORIZE_ENDPOINT},
        models::{
            error_response::ErrorResponse,
            success_response::SuccessResponse,
            ws::{
                market_data_feed_message::{MarketDataFeedMessage, MessageData, MethodType},
                portfolio_feed_request::PortfolioUpdateType,
                portfolio_feed_response::PortfolioFeedResponse,
                AuthorizeFeedResponse,
            },
        },
        protos::market_data_feed::FeedResponse as MarketDataFeedResponse,
        rate_limiter::RateLimitExceeded,
    },
    async_trait::async_trait,
    ezsockets::{Client as EzClient, ClientConfig, ClientExt, Error as EzError},
    protobuf::Message,
    reqwest::Url,
    serde_json,
    std::collections::{hash_set, HashSet},
    tokio::task::JoinHandle,
};

#[derive(Debug)]
pub struct PortfolioFeedClient<F>
where
    F: FnMut(PortfolioFeedResponse) + Send + Sync + 'static,
{
    pub handle: EzClient<Self>,
    callback: Option<F>,
}

#[derive(Debug)]
pub struct MarketDataFeedClient<F>
where
    F: FnMut(MarketDataFeedResponse) + Send + Sync + 'static,
{
    pub handle: EzClient<Self>,
    callback: Option<F>,
}

#[async_trait]
impl<F> ClientExt for PortfolioFeedClient<F>
where
    F: FnMut(PortfolioFeedResponse) + Send + Sync + 'static,
{
    type Call = ();

    async fn on_text(&mut self, text: String) -> Result<(), EzError> {
        if let Some(callback) = &mut self.callback {
            let data: PortfolioFeedResponse = serde_json::from_str::<PortfolioFeedResponse>(&text)?;
            callback(data);
        }
        Ok(())
    }

    async fn on_binary(&mut self, _: Vec<u8>) -> Result<(), EzError> {
        Ok(())
    }

    async fn on_call(&mut self, _: Self::Call) -> Result<(), EzError> {
        Ok(())
    }
}

#[async_trait]
impl<F> ClientExt for MarketDataFeedClient<F>
where
    F: FnMut(MarketDataFeedResponse) + Send + Sync + 'static,
{
    type Call = MarketDataCall;

    async fn on_text(&mut self, _: String) -> Result<(), EzError> {
        Ok(())
    }

    async fn on_binary(&mut self, binary_data: Vec<u8>) -> Result<(), EzError> {
        if let Some(callback) = &mut self.callback {
            let data: MarketDataFeedResponse =
                MarketDataFeedResponse::parse_from_bytes(&binary_data)?;
            callback(data);
        }
        Ok(())
    }

    async fn on_call(&mut self, call: Self::Call) -> Result<(), EzError> {
        let market_data_feed_message: MarketDataFeedMessage = MarketDataFeedMessage {
            guid: "someguid".to_string(),
            method: match call {
                MarketDataCall::SubscribeInstrument(_) => MethodType::Sub,
                MarketDataCall::ChangeMode(_) => MethodType::ChangeMode,
                MarketDataCall::UnsubscribeInstrument(_) => MethodType::Unsub,
            },
            data: match call {
                MarketDataCall::SubscribeInstrument(data) => data,
                MarketDataCall::ChangeMode(data) => data,
                MarketDataCall::UnsubscribeInstrument(data) => data,
            },
        };

        let message_text: String = serde_json::to_string(&market_data_feed_message).unwrap();
        let message_binary: &[u8] = message_text.as_bytes();
        self.handle.binary(message_binary)?;
        Ok(())
    }
}

impl<F, G> ApiClient<F, G>
where
    F: FnMut(PortfolioFeedResponse) + Send + Sync + 'static,
    G: FnMut(MarketDataFeedResponse) + Send + Sync + 'static,
{
    // Default update type is order only
    pub async fn connect_portfolio_feed(
        &mut self,
        update_types: Option<HashSet<PortfolioUpdateType>>,
        callback: Option<F>,
    ) -> Result<JoinHandle<()>, String> {
        let authorized_url: String = self
            .get_authorized_portfolio_feed_endpoint(update_types)
            .await
            .unwrap()
            .map_err(|_| "Failed to fetch Portfolio Feed WS URL".to_string())?
            .data
            .authorized_redirect_uri;

        let config: ClientConfig = ClientConfig::new(Url::parse(&authorized_url).unwrap());
        let (handle, future) =
            ezsockets::connect(|handle| PortfolioFeedClient { handle, callback }, config).await;
        self.portfolio_feed_client = Some(handle);

        let feed_future: JoinHandle<()> = tokio::spawn(async move {
            future.await.unwrap();
        });
        Ok(feed_future)
    }

    pub async fn connect_market_data_feed(
        &mut self,
        callback: Option<G>,
    ) -> Result<JoinHandle<()>, String> {
        let authorized_url: String = self
            .get_authorized_market_data_feed_endpoint()
            .await
            .unwrap()
            .map_err(|_| "Failed to fetch Market Data Feed WS URL".to_string())?
            .data
            .authorized_redirect_uri;

        let config: ClientConfig = ClientConfig::new(Url::parse(&authorized_url).unwrap());
        let (handle, future) =
            ezsockets::connect(|handle| MarketDataFeedClient { handle, callback }, config).await;
        self.market_data_feed_client = Some(handle);

        let feed_future: JoinHandle<()> = tokio::spawn(async move {
            future.await.unwrap();
        });
        Ok(feed_future)
    }

    pub async fn send_market_data_feed_message(
        &self,
        market_data_feed_message: MarketDataCall,
    ) -> Result<(), EzError> {
        if let Some(client) = &self.market_data_feed_client {
            client.call(market_data_feed_message)?;
        }
        Ok(())
    }

    pub async fn get_authorized_portfolio_feed_endpoint(
        &self,
        update_types: Option<HashSet<PortfolioUpdateType>>,
    ) -> Result<Result<SuccessResponse<AuthorizeFeedResponse>, ErrorResponse>, RateLimitExceeded>
    {
        let update_types: String = match update_types {
            Some(types) => {
                if types.is_empty() {
                    "order".to_string()
                } else {
                    let mut iter: hash_set::Iter<PortfolioUpdateType> = types.iter();
                    let mut temp: String = iter.next().unwrap().to_string();
                    for val in iter {
                        temp.push_str(",");
                        temp.push_str(&val.to_string());
                    }
                    temp
                }
            }
            None => "order".to_string(),
        };

        let res: reqwest::Response = self
            .get(
                WS_PORTFOLIO_FEED_AUTHORIZE_ENDPOINT,
                true,
                Some(&vec![("update_types".to_string(), update_types)]),
            )
            .await?;

        Ok(match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<AuthorizeFeedResponse>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        })
    }

    pub async fn get_authorized_market_data_feed_endpoint(
        &self,
    ) -> Result<Result<SuccessResponse<AuthorizeFeedResponse>, ErrorResponse>, RateLimitExceeded>
    {
        let res: reqwest::Response = self
            .get(WS_MARKET_DATA_FEED_AUTHORIZE_ENDPOINT, true, None)
            .await?;

        Ok(match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<AuthorizeFeedResponse>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        })
    }
}

#[derive(Debug)]
pub enum MarketDataCall {
    SubscribeInstrument(MessageData),
    ChangeMode(MessageData),
    UnsubscribeInstrument(MessageData),
    // Add other calls as needed
}

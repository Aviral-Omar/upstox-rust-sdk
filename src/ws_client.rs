use {
    crate::{
        client::ApiClient,
        constants::{
            APIVersion, BaseUrlType, WS_MARKET_DATA_FEED_AUTHORIZE_ENDPOINT,
            WS_PORTFOLIO_FEED_AUTHORIZE_ENDPOINT,
        },
        models::{
            error_response::ErrorResponse,
            success_response::SuccessResponse,
            ws::{
                AuthorizeFeedResponse,
                market_data_feed_v3_message::{
                    MarketDataFeedV3Message, MessageDataV3, MethodTypeV3,
                },
                portfolio_feed_request::PortfolioUpdateType,
                portfolio_feed_response::PortfolioFeedResponse,
            },
        },
        protos::market_data_feed_v3::FeedResponse as MarketDataFeedV3Response,
        rate_limiter::RateLimitExceeded,
    },
    async_trait::async_trait,
    ezsockets::{Bytes, Client as EzClient, ClientConfig, ClientExt, Error as EzError, Utf8Bytes},
    protobuf::Message,
    reqwest::Url,
    serde_json,
    std::collections::{HashSet, hash_set},
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
pub struct MarketDataFeedV3Client<F>
where
    F: FnMut(MarketDataFeedV3Response) + Send + Sync + 'static,
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

    async fn on_text(&mut self, text: Utf8Bytes) -> Result<(), EzError> {
        if let Some(callback) = &mut self.callback {
            let data: PortfolioFeedResponse = serde_json::from_str::<PortfolioFeedResponse>(&text)?;
            callback(data);
        }
        Ok(())
    }

    async fn on_binary(&mut self, _: Bytes) -> Result<(), EzError> {
        Ok(())
    }

    async fn on_call(&mut self, _: Self::Call) -> Result<(), EzError> {
        Ok(())
    }
}

#[async_trait]
impl<F> ClientExt for MarketDataFeedV3Client<F>
where
    F: FnMut(MarketDataFeedV3Response) + Send + Sync + 'static,
{
    type Call = MarketDataV3Call;

    async fn on_text(&mut self, _: Utf8Bytes) -> Result<(), EzError> {
        Ok(())
    }

    async fn on_binary(&mut self, binary_data: Bytes) -> Result<(), EzError> {
        if let Some(callback) = &mut self.callback {
            let data: MarketDataFeedV3Response =
                MarketDataFeedV3Response::parse_from_bytes(&binary_data)?;
            callback(data);
        }
        Ok(())
    }

    async fn on_call(&mut self, call: Self::Call) -> Result<(), EzError> {
        let market_data_feed_message: MarketDataFeedV3Message = MarketDataFeedV3Message {
            guid: "someguid".to_string(),
            method: match call {
                MarketDataV3Call::SubscribeInstrument(_) => MethodTypeV3::Sub,
                MarketDataV3Call::ChangeMode(_) => MethodTypeV3::ChangeMode,
                MarketDataV3Call::UnsubscribeInstrument(_) => MethodTypeV3::Unsub,
            },
            data: match call {
                MarketDataV3Call::SubscribeInstrument(data) => data,
                MarketDataV3Call::ChangeMode(data) => data,
                MarketDataV3Call::UnsubscribeInstrument(data) => data,
            },
        };

        let message_text: String = serde_json::to_string(&market_data_feed_message).unwrap();
        let message_binary: Vec<u8> = message_text.into_bytes();
        self.handle.binary(message_binary)?;
        Ok(())
    }
}

impl ApiClient {
    // Default update type is order only
    pub async fn connect_portfolio_feed(
        &mut self,
        update_types: Option<HashSet<PortfolioUpdateType>>,
        callback: Option<Box<dyn FnMut(PortfolioFeedResponse) + Send + Sync>>,
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

    pub async fn connect_market_data_feed_v3(
        &mut self,
        callback: Option<Box<dyn FnMut(MarketDataFeedV3Response) + Send + Sync>>,
    ) -> Result<JoinHandle<()>, String> {
        let authorized_url: String = self
            .get_authorized_market_data_feed_v3_endpoint()
            .await
            .unwrap()
            .map_err(|_| "Failed to fetch Market Data Feed V3 WS URL".to_string())?
            .data
            .authorized_redirect_uri;

        let config: ClientConfig = ClientConfig::new(Url::parse(&authorized_url).unwrap());
        let (handle, future) =
            ezsockets::connect(|handle| MarketDataFeedV3Client { handle, callback }, config).await;
        self.market_data_feed_v3_client = Some(handle);

        let feed_future: JoinHandle<()> = tokio::spawn(async move {
            future.await.unwrap();
        });
        Ok(feed_future)
    }

    pub async fn send_market_data_feed_v3_message(
        &self,
        market_data_feed_v3_message: MarketDataV3Call,
    ) -> Result<(), EzError> {
        if let Some(client) = &self.market_data_feed_v3_client {
            client.call(market_data_feed_v3_message)?;
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
                BaseUrlType::REGULAR,
                APIVersion::V2,
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

    pub async fn get_authorized_market_data_feed_v3_endpoint(
        &self,
    ) -> Result<Result<SuccessResponse<AuthorizeFeedResponse>, ErrorResponse>, RateLimitExceeded>
    {
        let res: reqwest::Response = self
            .get(
                WS_MARKET_DATA_FEED_AUTHORIZE_ENDPOINT,
                true,
                None,
                BaseUrlType::REGULAR,
                APIVersion::V3,
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
}

#[derive(Debug)]
pub enum MarketDataV3Call {
    SubscribeInstrument(MessageDataV3),
    ChangeMode(MessageDataV3),
    UnsubscribeInstrument(MessageDataV3),
    // Add other calls as needed
}

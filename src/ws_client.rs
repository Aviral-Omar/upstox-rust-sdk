use {
    crate::{
        client::ApiClient,
        constants::{WS_MARKET_DATA_FEED_AUTHORIZE_ENDPOINT, WS_PORTFOLIO_FEED_AUTHORIZE_ENDPOINT},
        models::{
            success_response::SuccessResponse,
            ws::{
                market_data_feed_message::{MarketDataFeedMessage, MessageData, MethodType},
                portfolio_feed_request::PortfolioUpdateType,
                portfolio_feed_response::PortfolioFeedResponse,
                AuthorizeFeedResponse,
            },
        },
        protos::market_data_feed::FeedResponse as MarketDataFeedResponse,
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

    async fn on_text(&mut self, text: String) -> Result<(), EzError> {
        println!("Market Data message: {}", text);
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
            guid: "".to_string(), // TODO check if this works
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
        self.handle
            .text(serde_json::to_string(&market_data_feed_message).unwrap())?;
        Ok(())
    }
}

impl<F, G> ApiClient<F, G>
where
    F: FnMut(PortfolioFeedResponse) + Send + Sync + 'static,
    G: FnMut(MarketDataFeedResponse) + Send + Sync + 'static,
{
    /* Default update type is order only */
    pub async fn connect_portfolio_feed(
        &self,
        update_types: Option<HashSet<PortfolioUpdateType>>,
        callback: Option<F>,
    ) -> Result<JoinHandle<()>, String> {
        let update_types: String = match update_types {
            Some(types) => {
                if types.is_empty() {
                    "order".to_string()
                } else {
                    let mut iter: hash_set::Iter<PortfolioUpdateType> = types.iter();
                    let mut temp: String = serde_json::to_string(iter.next().unwrap()).unwrap();
                    for val in iter {
                        temp.push_str(",");
                        temp.push_str(&serde_json::to_string(val).unwrap());
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
            .await;
        if res.status().as_u16() != 200 {
            return Err("Failed to fetch Portfolio Feed WS URL".to_string());
        }
        let authorized_url: String = res
            .json::<SuccessResponse<AuthorizeFeedResponse>>()
            .await
            .unwrap()
            .data
            .authorized_redirect_uri;

        let config: ClientConfig = ClientConfig::new(Url::parse(&authorized_url).unwrap());
        let (_handle, future) =
            ezsockets::connect(|handle| PortfolioFeedClient { handle, callback }, config).await;

        let feed_future: JoinHandle<()> = tokio::spawn(async move {
            future.await.unwrap();
        });
        Ok(feed_future)
    }

    pub async fn connect_market_data_feed(
        &self,
        callback: Option<G>,
    ) -> Result<JoinHandle<()>, String> {
        let res: reqwest::Response = self
            .get(WS_MARKET_DATA_FEED_AUTHORIZE_ENDPOINT, true, None)
            .await;
        if res.status().as_u16() != 200 {
            return Err("Failed to fetch Market Data Feed WS URL".to_string());
        }
        let authorized_url: String = res
            .json::<SuccessResponse<AuthorizeFeedResponse>>()
            .await
            .unwrap()
            .data
            .authorized_redirect_uri;

        let config: ClientConfig = ClientConfig::new(Url::parse(&authorized_url).unwrap());
        let (_handle, future) =
            ezsockets::connect(|handle| MarketDataFeedClient { handle, callback }, config).await;

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
            client.handle.call(market_data_feed_message)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum MarketDataCall {
    SubscribeInstrument(MessageData),
    ChangeMode(MessageData),
    UnsubscribeInstrument(MessageData),
    // Add other calls as needed
}
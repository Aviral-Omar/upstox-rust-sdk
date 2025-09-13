use {
    dotenvy::dotenv,
    futures::future::join_all,
    std::{collections::HashSet, env},
    tokio::signal,
    upstox_rust_sdk::{
        client::{ApiClient, AutomateLoginConfig, LoginConfig, MailProvider, WSConnectConfig},
        constants::UPLINK_API_KEY_ENV,
        models::ws::{
            market_data_feed_v3_message::{MessageDataV3, ModeTypeV3},
            portfolio_feed_request::PortfolioUpdateType,
            portfolio_feed_response::PortfolioFeedResponse,
        },
        protos::market_data_feed_v3::FeedResponse as MarketDataFeedV3Response,
        ws_client::MarketDataV3Call,
    },
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let _ = dotenv();

    let portfolio_feed_handler = |data: PortfolioFeedResponse| {
        println!("{:?}", data);
    };
    let market_data_feed_v3_handler = |data: MarketDataFeedV3Response| {
        println!("{:?}", data);
    };

    let api_key: String = env::var(UPLINK_API_KEY_ENV).unwrap();
    let (fetch_instruments, schedule_refresh_instruments) = (false, false);

    // ApiClient with websockets connected and handler specified
    let (api_client, tasks_vec) = ApiClient::new(
        &api_key,
        LoginConfig {
            authorize: true,
            automate_login_config: Some(AutomateLoginConfig {
                automate_login: true,
                schedule_login: false,
                automate_fetching_otp: true,
                mail_provider: Some(MailProvider::Google),
            }),
        },
        fetch_instruments,
        schedule_refresh_instruments,
        // Configuration to connect and handle websocket data.
        WSConnectConfig {
            connect_portfolio_stream: true,
            connect_market_data_stream_v3: true,
            // Select which portfolio data to fetch
            portfolio_stream_update_types: Some(HashSet::from([
                PortfolioUpdateType::Order,
                PortfolioUpdateType::Position,
                PortfolioUpdateType::Holding,
            ])),
            // Handle portfolio data feed
            portfolio_feed_callback: Some(Box::new(portfolio_feed_handler)),
            // Handle market data feed
            market_data_feed_v3_callback: Some(Box::new(market_data_feed_v3_handler)),
        },
    )
    .await
    .unwrap();

    let api_client = api_client.lock().await;
    api_client
        .send_market_data_feed_v3_message(MarketDataV3Call::SubscribeInstrument(MessageDataV3 {
            mode: ModeTypeV3::Full,
            instrument_keys: vec![
                "NSE_INDEX|NIFTY LARGEMID250".to_string(),
                "NSE_INDEX|Nifty Auto".to_string(),
                "NSE_INDEX|Nifty Midcap 50".to_string(),
            ],
        }))
        .await
        .unwrap();

    // This ensures that app continues running until the websockets die if connected or until SIGINT occurs
    tokio::select! {
        _ = join_all(tasks_vec) => {}
        _ = signal::ctrl_c() => {}
    };
}

use {
    dotenvy::dotenv,
    std::env,
    tokio::sync::MutexGuard,
    upstox_rust_sdk::{
        client::{ApiClient, AutomateLoginConfig, LoginConfig, MailProvider, WSConnectConfig},
        constants::UPLINK_API_KEY_ENV,
        models::{ws::portfolio_feed_response::PortfolioFeedResponse, ExchangeSegment},
        protos::market_data_feed::FeedResponse as MarketDataFeedResponse,
    },
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let _ = dotenv();

    let api_key: String = env::var(UPLINK_API_KEY_ENV).unwrap();
    let (fetch_instruments, schedule_refresh_instruments) = (true, true);

    // ApiClient which fetches instruments, schedules instruments refresh daily and stores it in ApiClient
    let (api_client, tasks_vec) = ApiClient::new(
        &api_key,
        LoginConfig {
            authorize: false,
            automate_login_config: Some(AutomateLoginConfig {
                automate_login: false,
                schedule_login: false,
                automate_fetching_otp: false,
                mail_provider: Some(MailProvider::Google),
            }),
        },
        /* Fetch all instruments data from UPSTOX and store it in the ApiClient. */
        fetch_instruments,
        /* Refresh instruments data daily at 6:30 AM. */
        schedule_refresh_instruments,
        WSConnectConfig {
            connect_portfolio_stream: false,
            connect_market_data_stream: false,
            portfolio_stream_update_types: None,
            portfolio_feed_callback: None::<Box<dyn FnMut(PortfolioFeedResponse) + Send + Sync>>,
            market_data_feed_callback: None::<Box<dyn FnMut(MarketDataFeedResponse) + Send + Sync>>,
        },
    )
    .await
    .unwrap();

    let api_client: MutexGuard<
        ApiClient<
            Box<dyn FnMut(PortfolioFeedResponse) + Send + Sync>,
            Box<dyn FnMut(MarketDataFeedResponse) + Send + Sync>,
        >,
    > = api_client.lock().await;
    print!(
        "{:?}",
        api_client
            .instruments
            .as_ref()
            .unwrap()
            .get(&ExchangeSegment::NseIndex)
            .unwrap()
            .get("INDEX")
            .unwrap()
    );
}

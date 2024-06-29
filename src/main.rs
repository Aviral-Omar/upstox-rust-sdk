use {
    dotenvy::dotenv,
    futures::future::join_all,
    std::{collections::HashSet, env},
    tokio::signal,
    upstox_rust_sdk::{
        client::{ApiClient, AutomateLoginConfig, LoginConfig, MailProvider, WSConnectConfig},
        constants::UPLINK_API_KEY_ENV,
        models::ws::{
            portfolio_feed_request::PortfolioUpdateType,
            portfolio_feed_response::PortfolioFeedResponse,
        },
        protos::market_data_feed::FeedResponse as MarketDataFeedResponse,
    },
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let _ = dotenv();

    let portfolio_feed_handler = |data: PortfolioFeedResponse| {
        println!("{:?}", data);
    };
    let market_data_feed_handler = |data: MarketDataFeedResponse| {
        println!("{:?}", data);
    };

    let api_key: String = env::var(UPLINK_API_KEY_ENV).unwrap();
    let (api_client, tasks_vec) = ApiClient::new(
        &api_key,
        LoginConfig {
            authorize: true,
            automate_login_config: Some(AutomateLoginConfig {
                automate_login: true,
                schedule_login: true,
                automate_fetching_otp: true,
                mail_provider: Some(MailProvider::Google),
            }),
        },
        false,
        false,
        WSConnectConfig {
            connect_portfolio_stream: true,
            connect_market_data_stream: true,
            portfolio_stream_update_types: Some(HashSet::from([
                PortfolioUpdateType::Order,
                PortfolioUpdateType::Position,
                PortfolioUpdateType::Holding,
            ])),
            portfolio_feed_callback: Some(portfolio_feed_handler),
            market_data_feed_callback: Some(market_data_feed_handler),
            // portfolio_feed_callback: None::<Box<dyn FnMut(PortfolioFeedResponse) + Send + Sync>>,
            // market_data_feed_callback: None::<Box<dyn FnMut(MarketDataFeedResponse) + Send + Sync>>,
        },
    )
    .await
    .unwrap();

    let api_client: tokio::sync::MutexGuard<ApiClient<_, _>> = api_client.lock().await;
    tokio::select! {
        _ = join_all(tasks_vec) => {}
        _ = signal::ctrl_c() => {}
    };
    // println!(
    //     "{:?}",
    //     api_client
    //         .instruments
    //         .as_ref()
    //         .unwrap()
    //         .get(&ExchangeSegment::NseIndex)
    //         .unwrap()
    //         .get("INDEX")
    //         .unwrap()
    // );
}
// ./geckodriver --binary "/home/aviralomar/.local/share/flatpak/exports/bin/org.mozilla.firefox" --profile-root "/home/aviralomar/.var/app/org.mozilla.firefox/cache/mozilla/firefox/cv70hco5.default-release"

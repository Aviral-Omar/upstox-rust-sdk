use {
    dotenvy::dotenv,
    futures::future::join_all,
    std::env,
    tokio::{signal, sync::MutexGuard},
    upstox_rust_sdk::{
        client::{ApiClient, AutomateLoginConfig, LoginConfig, MailProvider, WSConnectConfig},
        constants::UPLINK_API_KEY_ENV,
        models::{
            charges::brokerage_details_request::BrokerageDetailsRequest,
            success_response::SuccessResponse,
            user::{
                fund_and_margin_request::SegmentType,
                fund_and_margin_response::FundAndMarginResponse, profile_response::ProfileResponse,
            },
            ws::portfolio_feed_response::PortfolioFeedResponse,
            ProductType, TransactionType,
        },
        protos::market_data_feed::FeedResponse as MarketDataFeedResponse,
    },
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let _ = dotenv();

    let api_key: String = env::var(UPLINK_API_KEY_ENV).unwrap();
    let (fetch_instruments, schedule_refresh_instruments) = (false, false);

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

    {
        let api_client: MutexGuard<
            ApiClient<
                Box<dyn FnMut(PortfolioFeedResponse) + Send + Sync>,
                Box<dyn FnMut(MarketDataFeedResponse) + Send + Sync>,
            >,
        > = api_client.lock().await;

        // User Endpoints
        let _profile: SuccessResponse<ProfileResponse> = api_client.get_profile().await.unwrap();
        let _funds_and_margin: SuccessResponse<FundAndMarginResponse> = api_client
            .get_fund_and_margin(Some(SegmentType::Sec))
            .await
            .unwrap();

        // Charges Endpoints
        let _charges = api_client
            .get_brokerage_details(BrokerageDetailsRequest {
                instrument_token: "NSE_EQ|INE806T01012".to_string(),
                quantity: 2,
                product: ProductType::I,
                transaction_type: TransactionType::Buy,
                price: 1575.00,
            })
            .await
            .unwrap();

        // This is just for usage illustration. All endpoints in https://upstox.com/developer/api-documentation/open-api are available via the ApiClient.
    }

    // This ensures that app continues running until SIGINT occurs
    tokio::select! {
        _ = join_all(tasks_vec) => {}
        _ = signal::ctrl_c() => {}
    };
}

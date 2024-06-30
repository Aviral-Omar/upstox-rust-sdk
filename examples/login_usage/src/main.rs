use {
    dotenvy::dotenv,
    std::env,
    upstox_rust_sdk::{
        client::{ApiClient, AutomateLoginConfig, LoginConfig, MailProvider, WSConnectConfig},
        constants::UPLINK_API_KEY_ENV,
        models::ws::portfolio_feed_response::PortfolioFeedResponse,
        protos::market_data_feed::FeedResponse as MarketDataFeedResponse,
    },
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let _ = dotenv();

    let api_key: String = env::var(UPLINK_API_KEY_ENV).unwrap();
    let (fetch_instruments, schedule_refresh_instruments) = (false, false);

    // ApiClient which logs in automatically and schedules relogin daily when token expires
    let (api_client, tasks_vec) = ApiClient::new(
        &api_key,
        LoginConfig {
            authorize: true,
            automate_login_config: Some(AutomateLoginConfig {
                /* geckodriver or chromedriver binary must be running locally with port specified in env to use automatic login or schedule login.
                ./geckodriver --binary "~/.local/share/flatpak/exports/bin/org.mozilla.firefox" --profile-root "~/.var/app/org.mozilla.firefox/cache/mozilla/firefox/cv70hco5.default-release" */

                // Either GOOGLE_AUTHORIZATION_CODE environment must not be set or must be recent if using for the first time or refresh_token.txt has been deleted
                automate_login: true,
                /* Relogin is scheduled at 3:30 AM IST daily.*/
                schedule_login: true,
                /* Fetch OTP automatically from inbox by providing email IMAP access using env variables. For GMail, login page will be opened in your default web browsers and permission must be granted. */
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
}

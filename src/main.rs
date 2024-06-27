use {
    dotenvy::dotenv,
    std::env,
    upstox_rust_sdk::{
        client::{ApiClient, AutomateLoginConfig, LoginConfig, MailProvider},
        constants::UPLINK_API_KEY_ENV,
        models::ExchangeSegment,
    },
};

#[tokio::main]
async fn main() {
    let _ = dotenv();

    let api_key: String = env::var(UPLINK_API_KEY_ENV).unwrap();
    let api_client: std::sync::Arc<tokio::sync::Mutex<ApiClient>> = ApiClient::new(
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
    )
    .await
    .unwrap();
    let api_client = api_client.lock().await;

    println!(
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
// ./geckodriver --binary "/home/aviralomar/.local/share/flatpak/exports/bin/org.mozilla.firefox" --profile-root "/home/aviralomar/.var/app/org.mozilla.firefox/cache/mozilla/firefox/cv70hco5.default-release"

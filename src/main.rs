use {
    dotenvy::dotenv,
    std::env,
    uplink_sdk::{
        client::ApiClient,
        constants::{BASE_URL, UPLINK_API_KEY_ENV},
    },
};

#[tokio::main]
async fn main() {
    let _ = dotenv();

    let api_key: String = env::var(UPLINK_API_KEY_ENV).unwrap();
    let _client: ApiClient = ApiClient::new(BASE_URL, &api_key).await;
}
// ./geckodriver --binary "/home/aviralomar/.local/share/flatpak/exports/bin/org.mozilla.firefox" --profile-root "/home/aviralomar/.var/app/org.mozilla.firefox/cache/mozilla/firefox/cv70hco5.default-release"

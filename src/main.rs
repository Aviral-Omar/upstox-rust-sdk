use {
    dotenvy::dotenv,
    std::env,
    uplink_sdk::{
        client::ApiClient,
        constants::{BASE_URL, UPLINK_API_KEY_ENV},
        models::user::fund_and_margin_request::SegmentType,
    },
};

#[tokio::main]
async fn main() {
    let _ = dotenv();

    let api_key: String = env::var(UPLINK_API_KEY_ENV).unwrap();
    let api_client: ApiClient = ApiClient::new(BASE_URL, &api_key).await;

    println!("{:?}", api_client.get_profile().await);
    println!(
        "{:?}",
        api_client.get_fund_and_margin(Some(SegmentType::Sec)).await
    );
    println!(
        "{:?}",
        api_client
            .get_brokerage_details(
                "NSE_FO|49900".to_string(),
                10,
                "I".to_string(),
                "BUY".to_string(),
                575.0
            )
            .await
    )
}
// ./geckodriver --binary "/home/aviralomar/.local/share/flatpak/exports/bin/org.mozilla.firefox" --profile-root "/home/aviralomar/.var/app/org.mozilla.firefox/cache/mozilla/firefox/cv70hco5.default-release"

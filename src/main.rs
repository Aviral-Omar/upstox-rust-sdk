use {
    dotenvy::dotenv,
    std::env,
    upstox_rust_sdk::{
        client::ApiClient,
        constants::{BASE_URL, UPLINK_API_KEY_ENV},
        models::{
            charges::brokerage_details_request::BrokerageDetailsRequest,
            orders::{
                order_details_request::OrderDetailsRequest,
                trade_history_request::TradeHistoryRequest,
            },
            ProductType, SegmentType, TransactionType,
        },
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
        api_client
            .get_brokerage_details(BrokerageDetailsRequest {
                instrument_token: "NSE_FO|49900".to_string(),
                quantity: 10,
                product: ProductType::I,
                transaction_type: TransactionType::Buy,
                price: 575.0
            })
            .await
    );
    println!(
        "{:?}",
        api_client
            .get_order_details(OrderDetailsRequest {
                order_id: Some("1".to_string()),
                tag: None
            })
            .await
    );
    println!("{:?}", api_client.get_order_book().await);
    println!(
        "{:?}",
        api_client
            .get_trade_history(TradeHistoryRequest {
                segment: SegmentType::MF,
                start_date: "2024-01-01".to_string(),
                end_date: "2024-06-20".to_string(),
                page_number: 1,
                page_size: 1
            })
            .await
    );
}
// ./geckodriver --binary "/home/aviralomar/.local/share/flatpak/exports/bin/org.mozilla.firefox" --profile-root "/home/aviralomar/.var/app/org.mozilla.firefox/cache/mozilla/firefox/cv70hco5.default-release"

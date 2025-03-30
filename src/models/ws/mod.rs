pub mod market_data_feed_message;
pub mod market_data_feed_v3_message;
pub mod portfolio_feed_request;
pub mod portfolio_feed_response;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthorizeFeedResponse {
    pub authorized_redirect_uri: String,
}

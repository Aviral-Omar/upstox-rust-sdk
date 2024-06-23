use crate::{
    client::ApiClient,
    constants::CHARGES_BROKERAGE_DETAILS_ENDPOINT,
    models::{
        charges::{
            brokerage_details_request::BrokerageDetailsRequest,
            brokerage_details_response::BrokerageDetailsResponse,
        },
        error_response::ErrorResponse,
        success_response::SuccessResponse,
    },
    utils::ToKeyValueTuples,
};

impl ApiClient {
    pub async fn get_brokerage_details(
        &self,
        instrument_token: String,
        quantity: u32,
        product: String,
        transaction_type: String,
        price: f64,
    ) -> Result<SuccessResponse<BrokerageDetailsResponse>, ErrorResponse> {
        let brokerage_details_params: BrokerageDetailsRequest = BrokerageDetailsRequest {
            instrument_token,
            quantity,
            product,
            transaction_type,
            price,
        };

        let res: reqwest::Response = self
            .get(
                CHARGES_BROKERAGE_DETAILS_ENDPOINT,
                true,
                Some(brokerage_details_params.to_key_value_tuples_vec()),
            )
            .await;

        match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<BrokerageDetailsResponse>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        }
    }
}

use {
    crate::{
        client::ApiClient,
        constants::{APIVersion, BaseUrlType, CHARGES_BROKERAGE_DETAILS_ENDPOINT},
        models::{
            charges::{
                brokerage_details_request::BrokerageDetailsRequest,
                brokerage_details_response::BrokerageDetailsResponse,
            },
            error_response::ErrorResponse,
            success_response::SuccessResponse,
        },
        rate_limiter::RateLimitExceeded,
        utils::ToKeyValueTuples,
    },
    serde_valid::Validate,
};

impl ApiClient {
    pub async fn get_brokerage_details(
        &self,
        brokerage_details_params: BrokerageDetailsRequest,
    ) -> Result<Result<SuccessResponse<BrokerageDetailsResponse>, ErrorResponse>, RateLimitExceeded>
    {
        brokerage_details_params.validate().unwrap();
        let res: reqwest::Response = self
            .get(
                CHARGES_BROKERAGE_DETAILS_ENDPOINT,
                true,
                Some(&brokerage_details_params.to_key_value_tuples_vec()),
                BaseUrlType::REGULAR,
                APIVersion::V2,
            )
            .await?;

        Ok(match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<BrokerageDetailsResponse>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        })
    }
}

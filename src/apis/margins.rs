use {
    crate::{
        client::ApiClient,
        constants::{APIVersion, BaseUrlType, MARGINS_MARGIN_DETAILS_ENDPOINT},
        models::{
            error_response::ErrorResponse,
            margins::{
                margin_details_request::MarginDetailsRequest,
                margin_details_response::MarginDetailsResponse,
            },
            success_response::SuccessResponse,
        },
        rate_limiter::RateLimitExceeded,
        utils::ToKeyValueTuples,
    },
    serde_valid::Validate,
};

impl ApiClient {
    pub async fn get_margin_details(
        &self,
        margin_details_body: MarginDetailsRequest,
    ) -> Result<Result<SuccessResponse<MarginDetailsResponse>, ErrorResponse>, RateLimitExceeded>
    {
        margin_details_body.validate().unwrap();
        let res: reqwest::Response = self
            .post(
                MARGINS_MARGIN_DETAILS_ENDPOINT,
                true,
                Some(&margin_details_body.to_key_value_tuples_vec()),
                None,
                BaseUrlType::REGULAR,
                APIVersion::V2,
            )
            .await?;

        Ok(match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<MarginDetailsResponse>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        })
    }
}

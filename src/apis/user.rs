use crate::{
    client::ApiClient,
    constants::{
        APIVersion, BaseUrlType, USER_GET_FUND_AND_MARGIN_ENDPOINT, USER_GET_PROFILE_ENDPOINT,
    },
    models::{
        error_response::ErrorResponse,
        success_response::SuccessResponse,
        user::{
            fund_and_margin_request::{FundAndMarginRequest, SegmentType},
            fund_and_margin_response::FundAndMarginResponse,
            profile_response::ProfileResponse,
        },
    },
    rate_limiter::RateLimitExceeded,
    utils::ToKeyValueTuples,
};

impl ApiClient {
    pub async fn get_profile(
        &self,
    ) -> Result<Result<SuccessResponse<ProfileResponse>, ErrorResponse>, RateLimitExceeded> {
        let res: reqwest::Response = self
            .get(
                USER_GET_PROFILE_ENDPOINT,
                true,
                None,
                BaseUrlType::REGULAR,
                APIVersion::V2,
            )
            .await?;
        Ok(match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<ProfileResponse>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        })
    }

    pub async fn get_fund_and_margin(
        &self,
        segment: Option<SegmentType>,
    ) -> Result<Result<SuccessResponse<FundAndMarginResponse>, ErrorResponse>, RateLimitExceeded>
    {
        let fund_and_margin_params: FundAndMarginRequest = FundAndMarginRequest { segment };

        let res: reqwest::Response = self
            .get(
                USER_GET_FUND_AND_MARGIN_ENDPOINT,
                true,
                Some(&fund_and_margin_params.to_key_value_tuples_vec()),
                BaseUrlType::REGULAR,
                APIVersion::V2,
            )
            .await?;

        Ok(match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<FundAndMarginResponse>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        })
    }
}

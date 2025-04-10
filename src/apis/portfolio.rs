use {
    crate::{
        client::ApiClient,
        constants::{
            APIVersion, BaseUrlType, PORTFOLIO_CONVERT_POSITIONS_ENDPOINT,
            PORTFOLIO_HOLDINGS_ENDPOINT, PORTFOLIO_POSITIONS_ENDPOINT,
        },
        models::{
            error_response::ErrorResponse,
            portfolio::{
                convert_positions_request::ConvertPositionsRequest,
                convert_positions_response::ConvertPositionsResponse,
                holdings_response::HoldingsResponse, positions_response::PositionsResponse,
            },
            success_response::SuccessResponse,
        },
        rate_limiter::RateLimitExceeded,
    },
    serde_valid::Validate,
};

impl ApiClient {
    pub async fn get_positions(
        &self,
    ) -> Result<Result<SuccessResponse<Vec<PositionsResponse>>, ErrorResponse>, RateLimitExceeded>
    {
        let res: reqwest::Response = self
            .get(
                PORTFOLIO_POSITIONS_ENDPOINT,
                true,
                None,
                BaseUrlType::REGULAR,
                APIVersion::V2,
            )
            .await?;

        Ok(match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<Vec<PositionsResponse>>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        })
    }

    pub async fn convert_positions(
        &self,
        convert_positions_body: &ConvertPositionsRequest,
    ) -> Result<Result<SuccessResponse<ConvertPositionsResponse>, ErrorResponse>, RateLimitExceeded>
    {
        convert_positions_body.validate().unwrap();

        let res: reqwest::Response = self
            .put(
                PORTFOLIO_CONVERT_POSITIONS_ENDPOINT,
                true,
                Some(convert_positions_body),
                None,
                BaseUrlType::REGULAR,
                APIVersion::V2,
            )
            .await?;

        Ok(match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<ConvertPositionsResponse>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        })
    }

    pub async fn get_holdings(
        &self,
    ) -> Result<Result<SuccessResponse<Vec<HoldingsResponse>>, ErrorResponse>, RateLimitExceeded>
    {
        let res: reqwest::Response = self
            .get(
                PORTFOLIO_HOLDINGS_ENDPOINT,
                true,
                None,
                BaseUrlType::REGULAR,
                APIVersion::V2,
            )
            .await?;

        Ok(match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<Vec<HoldingsResponse>>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        })
    }
}

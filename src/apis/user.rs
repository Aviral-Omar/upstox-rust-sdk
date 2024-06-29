use crate::{
    client::ApiClient,
    constants::{USER_GET_FUND_AND_MARGIN_ENDPOINT, USER_GET_PROFILE_ENDPOINT},
    models::{
        error_response::ErrorResponse,
        success_response::SuccessResponse,
        user::{
            fund_and_margin_request::{FundAndMarginRequest, SegmentType},
            fund_and_margin_response::FundAndMarginResponse,
            profile_response::ProfileResponse,
        },
        ws::portfolio_feed_response::PortfolioFeedResponse,
    },
    protos::market_data_feed::FeedResponse as MarketDataFeedResponse,
    utils::ToKeyValueTuples,
};

impl<F, G> ApiClient<F, G>
where
    F: FnMut(PortfolioFeedResponse) + Send + Sync + 'static,
    G: FnMut(MarketDataFeedResponse) + Send + Sync + 'static,
{
    pub async fn get_profile(&self) -> Result<SuccessResponse<ProfileResponse>, ErrorResponse> {
        let res: reqwest::Response = self.get(USER_GET_PROFILE_ENDPOINT, true, None).await;
        match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<ProfileResponse>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        }
    }

    pub async fn get_fund_and_margin(
        &self,
        segment: Option<SegmentType>,
    ) -> Result<SuccessResponse<FundAndMarginResponse>, ErrorResponse> {
        let fund_and_margin_params: FundAndMarginRequest = FundAndMarginRequest { segment };

        let res: reqwest::Response = self
            .get(
                USER_GET_FUND_AND_MARGIN_ENDPOINT,
                true,
                Some(&fund_and_margin_params.to_key_value_tuples_vec()),
            )
            .await;

        match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<FundAndMarginResponse>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        }
    }
}

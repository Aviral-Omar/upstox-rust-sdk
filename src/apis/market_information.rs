use {
    crate::{
        client::ApiClient,
        constants::{
            MARKET_INFO_EXCHANGE_STATUS_ENDPOINT, MARKET_INFO_HOLIDAYS_ENDPOINT,
            MARKET_INFO_TIMINGS_ENDPOINT,
        },
        models::{
            error_response::ErrorResponse,
            market_information::{
                exchange_status_request::ExchangeStatusRequest,
                exchange_status_response::ExchangeStatusResponse,
                market_holidays_request::MarketHolidaysRequest,
                market_holidays_response::MarketHolidayResponse,
                market_timings_request::MarketTimingsRequest,
                market_timings_response::MarketTimingResponse,
            },
            success_response::SuccessResponse,
            ws::portfolio_feed_response::PortfolioFeedResponse,
        },
        protos::market_data_feed::FeedResponse as MarketDataFeedResponse,
    },
    serde_valid::Validate,
};

impl<F, G> ApiClient<F, G>
where
    F: FnMut(PortfolioFeedResponse) + Send + Sync + 'static,
    G: FnMut(MarketDataFeedResponse) + Send + Sync + 'static,
{
    pub async fn get_market_holidays(
        &self,
        market_holidays_path_params: MarketHolidaysRequest,
    ) -> Result<SuccessResponse<Vec<MarketHolidayResponse>>, ErrorResponse> {
        market_holidays_path_params.validate().unwrap();

        let res: reqwest::Response = self
            .get(
                format!(
                    "{}/{}",
                    MARKET_INFO_HOLIDAYS_ENDPOINT,
                    match market_holidays_path_params.date {
                        Some(date) => date,
                        None => "".to_string(),
                    }
                )
                .as_str(),
                false,
                None,
            )
            .await;

        match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<Vec<MarketHolidayResponse>>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        }
    }

    pub async fn get_market_timings(
        &self,
        market_timings_path_params: MarketTimingsRequest,
    ) -> Result<SuccessResponse<Vec<MarketTimingResponse>>, ErrorResponse> {
        market_timings_path_params.validate().unwrap();

        let res: reqwest::Response = self
            .get(
                format!(
                    "{}/{}",
                    MARKET_INFO_TIMINGS_ENDPOINT, market_timings_path_params.date
                )
                .as_str(),
                false,
                None,
            )
            .await;

        match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<Vec<MarketTimingResponse>>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        }
    }

    pub async fn get_exchange_status(
        &self,
        exchange_staus_path_params: ExchangeStatusRequest,
    ) -> Result<SuccessResponse<ExchangeStatusResponse>, ErrorResponse> {
        let res: reqwest::Response = self
            .get(
                format!(
                    "{}/{}",
                    MARKET_INFO_EXCHANGE_STATUS_ENDPOINT, exchange_staus_path_params.exchange
                )
                .as_str(),
                true,
                None,
            )
            .await;

        match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<ExchangeStatusResponse>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        }
    }
}

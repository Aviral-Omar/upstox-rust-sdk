use {
    crate::{
        client::ApiClient,
        constants::{
            APIVersion, BaseUrlType, HISTORICAL_CANDLE_DATA_ENDPOINT,
            HISTORICAL_CANDLE_INTRADAY_DATA_ENDPOINT,
        },
        models::{
            error_response::ErrorResponse,
            historical_data::{
                candle_data_response::CandleDataResponse,
                historical_candle_data_request::HistoricalCandleDataRequest,
                intraday_candle_data_request::IntradayCandleDataRequest,
            },
            success_response::SuccessResponse,
        },
        rate_limiter::RateLimitExceeded,
    },
    serde_valid::Validate,
};

impl ApiClient {
    pub async fn get_historical_candle_data(
        &self,
        historical_candles_path_params: HistoricalCandleDataRequest,
    ) -> Result<Result<SuccessResponse<CandleDataResponse>, ErrorResponse>, RateLimitExceeded> {
        historical_candles_path_params.validate().unwrap();

        let res: reqwest::Response = self
            .get(
                format!(
                    "{}/{}/{}/{}/{}",
                    HISTORICAL_CANDLE_DATA_ENDPOINT,
                    historical_candles_path_params.instrument_key,
                    historical_candles_path_params.interval,
                    historical_candles_path_params.to_date,
                    match historical_candles_path_params.from_date {
                        None => "".to_string(),
                        Some(from_date) => from_date,
                    }
                )
                .as_str(),
                false,
                None,
                BaseUrlType::REGULAR,
                APIVersion::V2,
            )
            .await?;

        Ok(match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<CandleDataResponse>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        })
    }

    pub async fn get_intraday_candle_data(
        &self,
        intraday_candles_path_params: IntradayCandleDataRequest,
    ) -> Result<Result<SuccessResponse<CandleDataResponse>, ErrorResponse>, RateLimitExceeded> {
        intraday_candles_path_params.validate().unwrap();

        let res: reqwest::Response = self
            .get(
                format!(
                    "{}/{}/{}",
                    HISTORICAL_CANDLE_INTRADAY_DATA_ENDPOINT,
                    intraday_candles_path_params.instrument_key,
                    intraday_candles_path_params.interval,
                )
                .as_str(),
                false,
                None,
                BaseUrlType::REGULAR,
                APIVersion::V2,
            )
            .await?;

        Ok(match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<CandleDataResponse>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        })
    }
}

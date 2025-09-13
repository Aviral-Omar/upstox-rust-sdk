use {
    crate::{
        client::ApiClient,
        constants::{
            APIVersion, BaseUrlType, MARKET_QUOTE_FULL_ENDPOINT, MARKET_QUOTE_LTP_ENDPOINT,
            MARKET_QUOTE_OHLC_ENDPOINT, MARKET_QUOTE_OPTION_GREEKS_ENDPOINT,
        },
        models::{
            error_response::ErrorResponse,
            market_quote::{
                full_market_quotes_request::FullMarketQuotesRequest,
                full_market_quotes_response::FullMarketQuotesResponse,
                ltp_quotes_request::LTPQuotesRequest, ltp_quotes_response::LTPQuotesResponse,
                ltp_quotes_v3_response::LTPQuotesV3Response,
                ohlc_quotes_request::OHLCQuotesRequest, ohlc_quotes_response::OHLCQuotesResponse,
                ohlc_quotes_v3_response::OHLCQuotesV3Response,
                option_greeks_request::OptionGreeksRequest,
                option_greeks_response::OptionGreeksResponse,
            },
            success_response::SuccessResponse,
        },
        rate_limiter::RateLimitExceeded,
        utils::ToKeyValueTuples,
    },
    serde_valid::Validate,
    std::collections::HashMap,
};

impl ApiClient {
    pub async fn get_full_market_quotes(
        &self,
        full_market_quotes_params: FullMarketQuotesRequest,
    ) -> Result<
        Result<SuccessResponse<HashMap<String, FullMarketQuotesResponse>>, ErrorResponse>,
        RateLimitExceeded,
    > {
        full_market_quotes_params.validate().unwrap();

        let res: reqwest::Response = self
            .get(
                MARKET_QUOTE_FULL_ENDPOINT,
                true,
                Some(&full_market_quotes_params.to_key_value_tuples_vec()),
                BaseUrlType::REGULAR,
                APIVersion::V2,
            )
            .await?;

        Ok(match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<HashMap<String, FullMarketQuotesResponse>>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        })
    }

    pub async fn get_ohlc_quotes_v3(
        &self,
        ohlc_quotes_params: OHLCQuotesRequest,
    ) -> Result<
        Result<SuccessResponse<HashMap<String, OHLCQuotesV3Response>>, ErrorResponse>,
        RateLimitExceeded,
    > {
        ohlc_quotes_params.validate().unwrap();

        let res: reqwest::Response = self
            .get(
                MARKET_QUOTE_OHLC_ENDPOINT,
                true,
                Some(&ohlc_quotes_params.to_key_value_tuples_vec()),
                BaseUrlType::REGULAR,
                APIVersion::V3,
            )
            .await?;

        Ok(match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<HashMap<String, OHLCQuotesV3Response>>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        })
    }

    #[deprecated(note = "Use get_ohlc_quotes_v3 instead")]
    pub async fn get_ohlc_quotes(
        &self,
        ohlc_quotes_params: OHLCQuotesRequest,
    ) -> Result<
        Result<SuccessResponse<HashMap<String, OHLCQuotesResponse>>, ErrorResponse>,
        RateLimitExceeded,
    > {
        ohlc_quotes_params.validate().unwrap();

        let res: reqwest::Response = self
            .get(
                MARKET_QUOTE_OHLC_ENDPOINT,
                true,
                Some(&ohlc_quotes_params.to_key_value_tuples_vec()),
                BaseUrlType::REGULAR,
                APIVersion::V2,
            )
            .await?;

        Ok(match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<HashMap<String, OHLCQuotesResponse>>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        })
    }

    #[deprecated(note = "Use get_ltp_quotes_v3 instead")]
    pub async fn get_ltp_quotes(
        &self,
        ltp_quotes_params: LTPQuotesRequest,
    ) -> Result<
        Result<SuccessResponse<HashMap<String, LTPQuotesResponse>>, ErrorResponse>,
        RateLimitExceeded,
    > {
        ltp_quotes_params.validate().unwrap();

        let res: reqwest::Response = self
            .get(
                MARKET_QUOTE_LTP_ENDPOINT,
                true,
                Some(&ltp_quotes_params.to_key_value_tuples_vec()),
                BaseUrlType::REGULAR,
                APIVersion::V2,
            )
            .await?;

        Ok(match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<HashMap<String, LTPQuotesResponse>>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        })
    }

    pub async fn get_ltp_quotes_v3(
        &self,
        ltp_quotes_params: LTPQuotesRequest,
    ) -> Result<
        Result<SuccessResponse<HashMap<String, LTPQuotesV3Response>>, ErrorResponse>,
        RateLimitExceeded,
    > {
        ltp_quotes_params.validate().unwrap();

        let res: reqwest::Response = self
            .get(
                MARKET_QUOTE_LTP_ENDPOINT,
                true,
                Some(&ltp_quotes_params.to_key_value_tuples_vec()),
                BaseUrlType::REGULAR,
                APIVersion::V3,
            )
            .await?;

        Ok(match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<HashMap<String, LTPQuotesV3Response>>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        })
    }

    pub async fn get_option_greeks(
        &self,
        option_greeks_params: OptionGreeksRequest,
    ) -> Result<
        Result<SuccessResponse<HashMap<String, OptionGreeksResponse>>, ErrorResponse>,
        RateLimitExceeded,
    > {
        option_greeks_params.validate().unwrap();

        let res: reqwest::Response = self
            .get(
                MARKET_QUOTE_OPTION_GREEKS_ENDPOINT,
                true,
                Some(&option_greeks_params.to_key_value_tuples_vec()),
                BaseUrlType::REGULAR,
                APIVersion::V3,
            )
            .await?;

        Ok(match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<HashMap<String, OptionGreeksResponse>>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        })
    }
}

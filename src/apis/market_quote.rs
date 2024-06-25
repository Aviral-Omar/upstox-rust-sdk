use {
    crate::{
        client::ApiClient,
        constants::{
            MARKET_QUOTE_FULL_ENDPOINT, MARKET_QUOTE_LTP_ENDPOINT, MARKET_QUOTE_OHLC_ENDPOINT,
        },
        models::{
            error_response::ErrorResponse,
            market_quote::{
                full_market_quotes_request::FullMarketQuotesRequest,
                full_market_quotes_response::FullMarketQuoteResponse,
                ltp_quotes_request::LTPQuotesRequest, ltp_quotes_response::LTPQuoteResponse,
                ohlc_quotes_request::OHLCQuotesRequest, ohlc_quotes_response::OHLCQuoteResponse,
            },
            success_response::SuccessResponse,
        },
        utils::ToKeyValueTuples,
    },
    serde_valid::Validate,
    std::collections::HashMap,
};

impl ApiClient {
    pub async fn get_full_market_quotes(
        &self,
        full_market_quotes_params: FullMarketQuotesRequest,
    ) -> Result<SuccessResponse<HashMap<String, FullMarketQuoteResponse>>, ErrorResponse> {
        full_market_quotes_params.validate().unwrap();

        let res: reqwest::Response = self
            .get(
                MARKET_QUOTE_FULL_ENDPOINT,
                true,
                Some(&full_market_quotes_params.to_key_value_tuples_vec()),
            )
            .await;

        match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<HashMap<String, FullMarketQuoteResponse>>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        }
    }

    pub async fn get_ohlc_quotes(
        &self,
        ohlc_quotes_params: OHLCQuotesRequest,
    ) -> Result<SuccessResponse<HashMap<String, OHLCQuoteResponse>>, ErrorResponse> {
        ohlc_quotes_params.validate().unwrap();

        let res: reqwest::Response = self
            .get(
                MARKET_QUOTE_OHLC_ENDPOINT,
                true,
                Some(&ohlc_quotes_params.to_key_value_tuples_vec()),
            )
            .await;

        match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<HashMap<String, OHLCQuoteResponse>>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        }
    }

    pub async fn get_ltp_quotes(
        &self,
        ltp_quotes_params: LTPQuotesRequest,
    ) -> Result<SuccessResponse<HashMap<String, LTPQuoteResponse>>, ErrorResponse> {
        ltp_quotes_params.validate().unwrap();

        let res: reqwest::Response = self
            .get(
                MARKET_QUOTE_LTP_ENDPOINT,
                true,
                Some(&ltp_quotes_params.to_key_value_tuples_vec()),
            )
            .await;

        match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<HashMap<String, LTPQuoteResponse>>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        }
    }
}

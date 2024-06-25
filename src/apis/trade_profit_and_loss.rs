use serde_valid::Validate;

use crate::{
    client::ApiClient,
    constants::{
        TRADE_PNL_REPORT_ENDPOINT, TRADE_PNL_REPORT_METADATA_ENDPOINT,
        TRADE_PNL_TRADES_CHARGES_ENDPOINT,
    },
    models::{
        error_response::ErrorResponse,
        success_response::SuccessResponse,
        trade_profit_and_loss::{
            pnl_report_meta_data_request::PnLReportMetaDataRequest,
            pnl_report_meta_data_response::PnLReportMetaDataResponse,
            profit_loss_request::ProfitAndLossRequest, profit_loss_response::ProfitAndLossResponse,
            trades_charges_request::TradesChargesRequest,
            trades_charges_response::TradesChargesResponse,
        },
    },
    utils::ToKeyValueTuples,
};

impl ApiClient {
    pub async fn get_pnl_report_metadata(
        &self,
        pnl_report_metadata_params: PnLReportMetaDataRequest,
    ) -> Result<SuccessResponse<PnLReportMetaDataResponse>, ErrorResponse> {
        pnl_report_metadata_params.validate().unwrap();

        let res: reqwest::Response = self
            .get(
                TRADE_PNL_REPORT_METADATA_ENDPOINT,
                true,
                Some(&pnl_report_metadata_params.to_key_value_tuples_vec()),
            )
            .await;

        match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<PnLReportMetaDataResponse>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        }
    }

    pub async fn get_pnl_report(
        &self,
        pnl_report_params: ProfitAndLossRequest,
    ) -> Result<SuccessResponse<Vec<ProfitAndLossResponse>>, ErrorResponse> {
        pnl_report_params.validate().unwrap();

        let res: reqwest::Response = self
            .get(
                TRADE_PNL_REPORT_ENDPOINT,
                true,
                Some(&pnl_report_params.to_key_value_tuples_vec()),
            )
            .await;

        match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<Vec<ProfitAndLossResponse>>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        }
    }

    pub async fn get_trades_charges(
        &self,
        trades_charges_params: TradesChargesRequest,
    ) -> Result<SuccessResponse<TradesChargesResponse>, ErrorResponse> {
        trades_charges_params.validate().unwrap();

        let res: reqwest::Response = self
            .get(
                TRADE_PNL_TRADES_CHARGES_ENDPOINT,
                true,
                Some(&trades_charges_params.to_key_value_tuples_vec()),
            )
            .await;

        match res.status().as_u16() {
            200 => Ok(res
                .json::<SuccessResponse<TradesChargesResponse>>()
                .await
                .unwrap()),
            _ => Err(res.json::<ErrorResponse>().await.unwrap()),
        }
    }
}

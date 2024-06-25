use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PnLReportMetaDataResponse {
    pub trades_count: u32,
    pub page_size_limit: u32,
}

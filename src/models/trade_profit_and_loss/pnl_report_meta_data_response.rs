use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct PnLReportMetaDataResponse {
    pub trades_count: u32,
    pub page_size_limit: u32,
}

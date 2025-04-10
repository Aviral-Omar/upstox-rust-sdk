use {
    crate::models::ResponseSummary,
    serde::{Deserialize, Serialize},
};

#[derive(Deserialize, Serialize, Debug)]
pub struct Page {
    pub page_number: u32,
    pub page_size: u32,
    pub total_records: Option<u32>,
    pub total_pages: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Metadata {
    pub page: Option<Page>,
    pub latency: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SuccessResponse<T> {
    pub status: String,
    pub data: T,
    pub errors: Option<()>,
    #[serde(alias = "metaData")]
    pub metadata: Option<Metadata>,
    pub summary: Option<ResponseSummary>,
}

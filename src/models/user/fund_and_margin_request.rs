use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum SegmentType {
    Sec,
    Com,
}

#[derive(Serialize, Debug)]
pub struct FundAndMarginRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment: Option<SegmentType>,
}

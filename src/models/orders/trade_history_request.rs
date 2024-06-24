use {crate::models::SegmentType, serde::Serialize, serde_valid::Validate};

#[derive(Serialize, Debug, Validate)]
pub struct TradeHistoryRequest {
    pub segment: SegmentType,
    #[validate(
        pattern = r"^\d{4}\-(0[1-9]|1[012])\-(0[1-9]|[12][0-9]|3[01])$",
        message = "start_date format must be yyyy-mm-dd"
    )]
    pub start_date: String,
    #[validate(
        pattern = r"^\d{4}\-(0[1-9]|1[012])\-(0[1-9]|[12][0-9]|3[01])$",
        message = "end_date format must be yyyy-mm-dd"
    )]
    pub end_date: String,
    #[validate(
        minimum = 1,
        message = "page_number must be greater than or equal to 1"
    )]
    pub page_number: u32,
    #[validate(minimum = 1, message = "page_size must be greater than or equal to 1")]
    pub page_size: u32,
}

use {serde::Serialize, serde_valid::Validate};

#[derive(Serialize, Debug, Validate)]
pub struct MarketTimingsRequest {
    #[validate(
        pattern = r"^\d{4}\-(0[1-9]|1[012])\-(0[1-9]|[12][0-9]|3[01])$",
        message = "date format must be yyyy-mm-dd"
    )]
    pub date: String,
}

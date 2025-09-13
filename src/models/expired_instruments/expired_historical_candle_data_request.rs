use {serde::Serialize, serde_valid::Validate};

#[derive(Serialize, Debug, Validate)]
pub struct ExpiredHistoricalCandleDataRequest {
    #[validate(
        pattern = r"^(?:NSE_EQ|NSE_FO|NCD_FO|BSE_EQ|BSE_FO|BCD_FO|MCX_FO|NSE_INDEX|BSE_INDEX|MCX_INDEX|NSE_COM)\|[\w\d\-]+\|(0[1-9]|[12]\d|3[01])-(0[1-9]|1[012])-(\d{4})$",
        message = "Invalid expired_instrument_key"
    )]
    pub expired_instrument_key: String,
    #[validate(enumerate = ["1minute", "3minute", "5minute", "15minute", "30minute", "day"])]
    pub interval: String, // Can be 1minute, 3minute, 5minute, 15minute, 30minute, day
    #[validate(
        pattern = r"^\d{4}\-(0[1-9]|1[012])\-(0[1-9]|[12][0-9]|3[01])$",
        message = "to_date format must be yyyy-mm-dd"
    )]
    pub to_date: String,
    #[validate(
        pattern = r"^\d{4}\-(0[1-9]|1[012])\-(0[1-9]|[12][0-9]|3[01])$",
        message = "from_date format must be yyyy-mm-dd"
    )]
    pub from_date: String,
}

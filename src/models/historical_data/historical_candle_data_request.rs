use {serde::Serialize, serde_valid::Validate};

#[derive(Serialize, Debug, Validate)]
pub struct HistoricalCandleDataRequest {
    #[validate(
        pattern = r"^(?:^NSE_EQ|NSE_FO|NCD_FO|BSE_EQ|BSE_FO|BCD_FO|MCX_FO|NSE_INDEX|BSE_INDEX|MCX_INDEX)\|[\w ]+(,(?:NSE_EQ|NSE_FO|NCD_FO|BSE_EQ|BSE_FO|BCD_FO|MCX_FO|NSE_INDEX|BSE_INDEX|MCX_INDEX)\|[\w ]+)*?$",
        message = "Invalid instrument_key"
    )]
    pub instrument_key: String,
    #[validate(enumerate = ["1minute", "30minute", "day", "week", "month"])]
    pub interval: String, // Can be 1minute, 30minute, day, week or month
    #[validate(
        pattern = r"^\d{4}\-(0[1-9]|1[012])\-(0[1-9]|[12][0-9]|3[01])$",
        message = "to_date format must be yyyy-mm-dd"
    )]
    pub to_date: String,
    #[validate(
        pattern = r"^\d{4}\-(0[1-9]|1[012])\-(0[1-9]|[12][0-9]|3[01])$",
        message = "from_date format must be yyyy-mm-dd"
    )]
    pub from_date: Option<String>,
}

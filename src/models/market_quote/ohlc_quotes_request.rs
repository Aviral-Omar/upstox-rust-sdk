use {serde::Serialize, serde_valid::Validate};

#[derive(Serialize, Debug, Validate)]
pub struct OHLCQuotesRequest {
    #[validate(
        pattern = r"^(?:^NSE_EQ|NSE_FO|NCD_FO|BSE_EQ|BSE_FO|BCD_FO|MCX_FO|NSE_INDEX|BSE_INDEX|MCX_INDEX)\|[\w ]+(,(?:NSE_EQ|NSE_FO|NCD_FO|BSE_EQ|BSE_FO|BCD_FO|MCX_FO|NSE_INDEX|BSE_INDEX|MCX_INDEX)\|[\w ]+)*?$",
        message = "Invalid instrument_key"
    )]
    pub instrument_key: String,
    #[validate(enumerate = ["1d", "I1", "I30"])]
    pub interval: String,
}

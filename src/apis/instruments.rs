use {
    crate::{
        client::ApiClient,
        constants::{
            INSTRUMENTS_ARCHIVE_FILENAME, INSTRUMENTS_COMPLETE_URL, INSTRUMENTS_JSON_FILENAME,
        },
        models::{
            instruments::instruments_response::InstrumentsResponse,
            ws::portfolio_feed_response::PortfolioFeedResponse, ExchangeSegment,
        },
        protos::market_data_feed::FeedResponse as MarketDataFeedResponse,
    },
    flate2::read::GzDecoder,
    reqwest::{Client, Response},
    std::{
        collections::HashMap,
        fs::File,
        io::{copy, Read},
    },
    tokio::fs,
    tracing::info,
};

impl<F, G> ApiClient<F, G>
where
    F: FnMut(PortfolioFeedResponse) + Send + Sync + 'static,
    G: FnMut(MarketDataFeedResponse) + Send + Sync + 'static,
{
    pub async fn get_instruments(&self) -> Result<Vec<InstrumentsResponse>, String> {
        let client: &Client = &self.client;
        let archive_path: &str = INSTRUMENTS_ARCHIVE_FILENAME;
        let json_path: &str = INSTRUMENTS_JSON_FILENAME;
        let url: &str = INSTRUMENTS_COMPLETE_URL;

        let archive_file: File = match File::open(archive_path) {
            Ok(file) => Ok(file),
            Err(_) => {
                let response: Response = client
                    .get(url)
                    .send()
                    .await
                    .map_err(|_| "Failed to fetch instruments".to_string())?;
                let bytes = response
                    .bytes()
                    .await
                    .map_err(|_| "Failed to read response bytes".to_string())?;
                fs::write(archive_path, &bytes)
                    .await
                    .map_err(|_| "Failed to write archive".to_string())?;
                File::open(archive_path).map_err(|_| "Failed to open archive".to_string())
            }
        }?;
        info!("Instruments archive downloaded");

        let mut archive: GzDecoder<File> = GzDecoder::new(archive_file);
        let mut output_file: File =
            File::create(json_path).map_err(|_| "Failed to create JSON file".to_string())?;
        copy(&mut archive, &mut output_file)
            .map_err(|_| "Failed to extract archive".to_string())?;

        fs::remove_file(archive_path)
            .await
            .expect("Failed to delete archive");

        let mut json_file: File =
            File::open(json_path).map_err(|_| "Failed to open JSON file".to_string())?;
        let mut json_content: String = String::new();
        json_file
            .read_to_string(&mut json_content)
            .map_err(|_| "Failed to read JSON file")?;
        fs::remove_file(json_path)
            .await
            .map_err(|_| "Failed to delete JSON file")?;

        let instruments_data: Vec<InstrumentsResponse> = serde_json::from_str(&json_content)
            .map_err(|_| "Failed to parse Instruments JSON".to_string())?;
        Ok(instruments_data)
    }

    pub fn parse_instruments(
        instruments: Vec<InstrumentsResponse>,
    ) -> HashMap<ExchangeSegment, HashMap<String, Vec<InstrumentsResponse>>> {
        let mut map: HashMap<ExchangeSegment, HashMap<String, Vec<InstrumentsResponse>>> =
            HashMap::new();

        for instrument in instruments {
            let (segment, instrument_type) = match &instrument {
                InstrumentsResponse::EquityResponse {
                    segment,
                    instrument_type,
                    ..
                } => (segment.clone(), instrument_type.clone()),
                InstrumentsResponse::DerivativeResponse {
                    segment,
                    instrument_type,
                    ..
                } => (segment.clone(), instrument_type.clone()),
                InstrumentsResponse::IndexResponse {
                    segment,
                    instrument_type,
                    ..
                } => (segment.clone(), instrument_type.clone()),
                InstrumentsResponse::CommodityResponse {
                    segment,
                    instrument_type,
                    ..
                } => (segment.clone(), instrument_type.clone()),
            };

            let segment_map: &mut HashMap<String, Vec<InstrumentsResponse>> =
                map.entry(segment).or_insert_with(HashMap::new);
            segment_map
                .entry(instrument_type)
                .or_insert_with(Vec::new)
                .push(instrument);
        }

        map
    }
}

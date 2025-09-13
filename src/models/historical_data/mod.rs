pub mod candle_data_response;
pub mod historical_candle_data_request;
pub mod historical_candle_data_v3_request;
pub mod intraday_candle_data_request;
pub mod intraday_candle_data_v3_request;

use {
    serde::{Deserialize, Serialize},
    serde_valid::validation::Error,
};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Unit {
    Minutes,
    Hours,
    Days,
    Weeks,
    Months,
}

pub(super) fn validate_interval(unit: &Unit, interval: &str) -> Result<(), Error> {
    let interval_num = interval
        .parse::<u32>()
        .map_err(|_| Error::Custom("interval must be a valid number".to_string()));
    match interval_num {
        Ok(num) => match unit {
            Unit::Minutes => {
                if num >= 1 && num <= 300 {
                    Ok(())
                } else {
                    Err(Error::Custom(
                        "interval must be between 1 and 300".to_string(),
                    ))
                }
            }
            Unit::Hours => {
                if num >= 1 && num <= 5 {
                    Ok(())
                } else {
                    Err(Error::Custom(
                        "interval must be between 1 and 5".to_string(),
                    ))
                }
            }
            Unit::Days | Unit::Weeks | Unit::Months => {
                if num == 1 {
                    Ok(())
                } else {
                    Err(Error::Custom(
                        "interval must be 1 for days, weeks or months units".to_string(),
                    ))
                }
            }
        },
        Err(err) => Err(err),
    }
}

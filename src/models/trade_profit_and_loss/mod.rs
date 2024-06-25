pub mod pnl_report_meta_data_request;
pub mod pnl_report_meta_data_response;
pub mod profit_loss_request;
pub mod profit_loss_response;
pub mod trades_charges_request;
pub mod trades_charges_response;

use {crate::models::SegmentType, serde_valid::validation};

fn segment_validation(segment: &SegmentType) -> Result<(), validation::Error> {
    match segment != &SegmentType::MF {
        true => Ok(()),
        false => Err(validation::Error::Custom(
            "segment cannot be MF".to_string(),
        )),
    }
}

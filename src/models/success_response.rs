use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SuccessResponse<T> {
    pub status: String,
    pub data: T,
}

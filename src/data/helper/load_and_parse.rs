// load and parse any RON structure - the centralised helper

use serde::de::DeserializeOwned;
use std::fs;
use std::path::Path;

use crate::data::DataError;

pub fn load_and_parse<T: DeserializeOwned>(path: &Path) -> Result<T, DataError> {
    let content = fs::read_to_string(path)?; // becomes DataError::Io
    let data: T = ron::from_str(&content)?; // becomes DataError::Ron

    Ok(data)
}

// build a key from the RON file

use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use thiserror::Error;

use crate::data_handling::key_ref::KeyRef;
use crate::globals::data_constants::KEYS_PATH;
//  setup error handling

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("RON error at: {0}")]
    Ron(#[from] ron::error::SpannedError),
}

//  bow read our Keypads structure from the RON file

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Key {
    pub id: String,
    pub label: String,
    pub tooltip_text: String,
    pub qalc_term: String,
}

impl Key {
    pub fn from_ron(key_ref: &KeyRef) -> Result<Self, Error> {
        let key_path = format!("{}{}/{}.ron", KEYS_PATH, &key_ref.library, &key_ref.id);
        let data_path = Path::new(&key_path);
        let key_ron = fs::read_to_string(&data_path)?;
        let key: Key = ron::from_str(&key_ron)?;

        Ok(key)
    }
}

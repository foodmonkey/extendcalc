// build a keypad from the RON file

use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use thiserror::Error;

use crate::data_handling::key_ref::KeyRef;
use crate::data_handling::keypad_ref::KeypadRef;

use crate::globals::data_constants::KEYPADS_PATH;

//  setup error handling

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("RON error at: {0}")]
    Ron(#[from] ron::error::SpannedError),
}

//  bow read our Keypads structure from the RON file

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct Keypad {
    pub id: String,
    pub label: String,
    pub tooltip: String,
    pub rows: u32,
    pub columns: u32,
    pub keys: Vec<KeyRef>,
}

impl Keypad {
    pub fn from_ron(keypad_ref: &KeypadRef) -> Result<Self, Error> {
        let keypad_path = format!("{}/{}.ron", KEYPADS_PATH, &keypad_ref.id);
        let data_path = Path::new(&keypad_path);
        let keypad_ron = fs::read_to_string(&data_path)?;
        let keypad = ron::from_str(&keypad_ron)?;

        Ok(keypad)
    }
}

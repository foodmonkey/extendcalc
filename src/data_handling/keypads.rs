// build the keypads list from the RON file (either from the application keypads
// or from the library keypads)

use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use thiserror::Error;

use crate::data_handling::keypad_ref::KeypadRef;

//  setup error handling

pub type KeypadsResult = Result<Keypads, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("RON error at: {0}")]
    Ron(#[from] ron::error::SpannedError),
}

//  bow read our Keypads structure from the RON file

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct Keypads {
    pub keypads: Vec<KeypadRef>,
}

impl Keypads {
    pub fn from_ron(keypads_path: &str) -> Result<Self, Error> {
        let data_path = Path::new(keypads_path);
        let keypads_ron = fs::read_to_string(&data_path)?;
        let keypads = ron::from_str(&keypads_ron)?;

        Ok(keypads)
    }
}

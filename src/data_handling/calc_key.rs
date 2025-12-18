// build a key from the RON file

use ron::error;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::path::Path;

use crate::data_handling::models::KeyRef;
use crate::globals::ui_constants::KEYS_PATH;
//  setup error handling

#[derive(Debug)]
pub enum CalcKeyError {
    Io(std::io::Error),
    Ron(error::SpannedError),
}

impl fmt::Display for CalcKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalcKeyError::Io(err) => write!(f, "IO error: {}", err),
            CalcKeyError::Ron(err) => write!(f, "RON error at: {}", err),
        }
    }
}

impl std::error::Error for CalcKeyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CalcKeyError::Io(err) => Some(err),
            CalcKeyError::Ron(err) => Some(err),
        }
    }
}

impl From<std::io::Error> for CalcKeyError {
    fn from(err: std::io::Error) -> Self {
        CalcKeyError::Io(err)
    }
}

impl From<ron::error::SpannedError> for CalcKeyError {
    fn from(err: ron::error::SpannedError) -> Self {
        CalcKeyError::Ron(err)
    }
}

//  bow read our Keypads structure from the RON file

#[derive(Debug, Deserialize, Serialize, Clone)]
pub type CalcKey = KeyDef;

impl CalcKey {
    pub fn from_ron(key_ref: &KeyRef) -> Result<Self, CalcKeyError> {
        let key_path = format!("{}{}/{}.ron", KEYS_PATH, &key_ref.library, &key_ref.id);
        let data_path = Path::new(&key_path);
        let key_ron = fs::read_to_string(&data_path)?;
        let key: CalcKey = ron::from_str(&key_ron)?;

        Ok(key)
    }
}

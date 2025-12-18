// build a keypad from the RON file

use ron::error;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::path::Path;

use crate::data_handling::models::{KeyRef, KeypadRef};
use crate::globals::ui_constants::KEYPADS_PATH;

//  setup error handling

#[derive(Debug)]
pub enum CalcKeypadError {
    Io(std::io::Error),
    Ron(error::SpannedError),
}

impl fmt::Display for CalcKeypadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalcKeypadError::Io(err) => write!(f, "IO error: {}", err),
            CalcKeypadError::Ron(err) => {
                write!(f, "RON error at: {}", err)
            }
        }
    }
}

impl std::error::Error for CalcKeypadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CalcKeypadError::Io(err) => Some(err),
            CalcKeypadError::Ron(err) => Some(err),
        }
    }
}
impl From<std::io::Error> for CalcKeypadError {
    fn from(err: std::io::Error) -> Self {
        CalcKeypadError::Io(err)
    }
}

impl From<error::SpannedError> for CalcKeypadError {
    fn from(err: error::SpannedError) -> Self {
        CalcKeypadError::Ron(err)
    }
}

//  bow read our Keypads structure from the RON file

#[derive(Debug, Deserialize, Serialize, Clone)]
pub type CalcKeypad = KeypadDef;

impl CalcKeypad {
    pub fn from_ron(keypad_ref: &KeypadRef) -> Result<Self, CalcKeypadError> {
        let keypad_path = format!("{}/{}.ron", KEYPADS_PATH, &keypad_ref.id);
        let data_path = Path::new(&keypad_path);
        let keypad_ron = fs::read_to_string(&data_path)?;
        let keypad: CalcKeypad = ron::from_str(&keypad_ron)?;

        Ok(keypad)
    }
}

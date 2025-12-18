// build the keypads list from the RON file (either from the application keypads
// or from the library keypads)

use ron::error;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::path::Path;

use crate::data_handling::models::KeypadRef;

//  setup error handling

#[derive(Debug)]
pub enum CalcKeypadsError {
    Io(std::io::Error),
    Ron(error::SpannedError),
}

impl fmt::Display for CalcKeypadsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalcKeypadsError::Io(err) => write!(f, "IO error: {}", err),
            CalcKeypadsError::Ron(err) => {
                write!(f, "RON error at: {}", err)
            }
        }
    }
}

impl std::error::Error for CalcKeypadsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CalcKeypadsError::Io(err) => Some(err),
            CalcKeypadsError::Ron(err) => Some(err),
        }
    }
}

impl From<std::io::Error> for CalcKeypadsError {
    fn from(err: std::io::Error) -> Self {
        CalcKeypadsError::Io(err)
    }
}

impl From<error::SpannedError> for CalcKeypadsError {
    fn from(err: error::SpannedError) -> Self {
        CalcKeypadsError::Ron(err)
    }
}

//  bow read our Keypads structure from the RON file

#[derive(Debug, Deserialize, Serialize, Clone)]
pub type CalcKeypads = Keypads;

impl CalcKeypads {
    pub fn from_ron(keypads_path: &str) -> Result<Self, CalcKeypadsError> {
        let data_path = Path::new(keypads_path);
        let keypads_ron = fs::read_to_string(&data_path)?;
        let keypads: CalcKeypads = ron::from_str(&keypads_ron)?;

        Ok(keypads)
    }
}

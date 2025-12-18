// build an assembled keyboard

use crate::data_handling::calc_keygrid::{CalcKeyGrid, CalcKeyGridError};
use crate::data_handling::calc_keypad::{CalcKeypad, CalcKeypadError};
use crate::data_handling::models::KeypadRef;
use serde::{Deserialize, Serialize};
use std::fmt;

//  set up our error handling

#[derive(Debug)]
pub enum AssembledKeypadError {
    FailedCalcKeypad(CalcKeypadError),
    FailedCalcKeyGrid(CalcKeyGridError),
}

impl fmt::Display for AssembledKeypadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AssembledKeypadError::FailedCalcKeypad(err) => {
                write!(f, "Failed to build CalcKeypad: {}", err)
            }
            AssembledKeypadError::FailedCalcKeyGrid(err) => {
                write!(f, "Failed to build CalcGrid: {}", err)
            }
        }
    }
}

impl std::error::Error for AssembledKeypadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AssembledKeypadError::FailedCalcKeypad(err) => Some(err),
            AssembledKeypadError::FailedCalcKeyGrid(err) => Some(err),
        }
    }
}

impl From<CalcKeypadError> for AssembledKeypadError {
    fn from(err: CalcKeypadError) -> Self {
        AssembledKeypadError::FailedCalcKeypad(err)
    }
}

impl From<CalcKeyGridError> for AssembledKeypadError {
    fn from(err: CalcKeyGridError) -> Self {
        AssembledKeypadError::FailedCalcKeyGrid(err)
    }
}

//  bow read our Keypads structure from the RON file

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AssembledKeypad {
    pub keypad: CalcKeypad,
    pub keygrid: CalcKeyGrid,
}

impl AssembledKeypad {
    pub fn new(keypad_ref: &KeypadRef) -> Result<Self, AssembledKeypadError> {
        let loaded_keypad = CalcKeypad::from_ron(keypad_ref)?;
        let loaded_grid = CalcKeyGrid::new(&loaded_keypad.keys)?;

        println!("keypad {} assembled", keypad_ref.id);
        Ok(AssembledKeypad {
            keypad: loaded_keypad,
            keygrid: loaded_grid,
        })
    }
}

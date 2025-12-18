// struct to hold all the keypad tree
// we build this from the json files and then use it
// to construct the ui - it gets saved in the config info
// so we don't have to read the JSON files every time at startup

use serde::{Deserialize, Serialize};
use std::fmt;

use crate::data_handling::assembled_keypad::{AssembledKeypad, AssembledKeypadError};
use crate::data_handling::calc_keypads::{CalcKeypads, CalcKeypadsError};
use crate::globals::ui_constants::{UI_KEYPADS_LIST_RON, UI_PATH};

//  error handling

#[derive(Debug)]
pub enum AssembledKeypadsError {
    FailedCalcKeypads(CalcKeypadsError),
    FailedAssembledKeypad(AssembledKeypadError),
}

impl fmt::Display for AssembledKeypadsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AssembledKeypadsError::FailedCalcKeypads(err) => {
                write!(f, "Failed to build CalcKeypads: {}", err)
            }
            AssembledKeypadsError::FailedAssembledKeypad(err) => {
                write!(f, "Failed to build AssembledKeypad: {}", err)
            }
        }
    }
}

impl std::error::Error for AssembledKeypadsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AssembledKeypadsError::FailedCalcKeypads(err) => Some(err),
            AssembledKeypadsError::FailedAssembledKeypad(err) => Some(err),
        }
    }
}

impl From<CalcKeypadsError> for AssembledKeypadsError {
    fn from(err: CalcKeypadsError) -> Self {
        AssembledKeypadsError::FailedCalcKeypads(err)
    }
}

impl From<AssembledKeypadError> for AssembledKeypadsError {
    fn from(err: AssembledKeypadError) -> Self {
        AssembledKeypadsError::FailedAssembledKeypad(err)
    }
}

//  the struct and implmentation code

#[derive(Debug, Deserialize, Serialize)]
pub struct AssembledKeypads {
    pub keypads_list: CalcKeypads,
    pub assembled_keypads: Vec<AssembledKeypad>,
}

// now we read the list of keypad refs from the keypads JSON file

impl AssembledKeypads {
    pub fn new() -> Result<Self, AssembledKeypadsError> {
        //  load the list of keypads for the UI

        let keypads_list_path = format!("{}{}", UI_PATH, UI_KEYPADS_LIST_RON);
        let loaded_keypads_list = CalcKeypads::from_ron(&keypads_list_path)?;
        let mut loaded_keypads: Vec<AssembledKeypad> = Vec::new();

        //  then loop through the list and assemble each keyboard

        for keypad_ref in &loaded_keypads_list.keypads {
            let assembled_keypad = AssembledKeypad::new(&keypad_ref)?;
            loaded_keypads.push(assembled_keypad);
            println!("assembled keypad {} pushed", keypad_ref.id);
        }

        Ok(AssembledKeypads {
            keypads_list: loaded_keypads_list,
            assembled_keypads: loaded_keypads,
        })
    }
}

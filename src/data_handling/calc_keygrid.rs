// build a keygrid from the RON files for a keypad

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

use crate::data_handling::calc_key::{CalcKey, CalcKeyError};
use crate::data_handling::models::KeyRef;

//  setup error handling

#[derive(Debug)]
pub enum CalcKeyGridError {
    FailedCalcKey(CalcKeyError),
}

impl fmt::Display for CalcKeyGridError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalcKeyGridError::FailedCalcKey(err) => {
                write!(f, "Failed to create CalcKey: {}", err)
            }
        }
    }
}

impl std::error::Error for CalcKeyGridError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CalcKeyGridError::FailedCalcKey(err) => Some(err),
        }
    }
}

impl From<CalcKeyError> for CalcKeyGridError {
    fn from(err: CalcKeyError) -> Self {
        CalcKeyGridError::FailedCalcKey(err)
    }
}

//  now assemble all the keys for a keypad into a grid
// ysing a hashmap with a tuple as key so i can referencw
// individual cells - the grid will render out a collection
// row widgets that will display actal buttons in a grid on screen

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq, Clone)]
pub struct GridPosition {
    pub row: u32,
    pub column: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CalcKeyGrid {
    pub cells: HashMap<GridPosition, CalcKey>,
}

impl CalcKeyGrid {
    pub fn new(keyref_list: &Vec<KeyRef>) -> Result<Self, CalcKeyGridError> {
        let mut assembled_cells = HashMap::<GridPosition, CalcKey>::new();

        for key_ref in keyref_list {
            let loaded_key = CalcKey::from_ron(&key_ref)?;

            let grid_position = GridPosition {
                row: loaded_key.row,
                column: loaded_key.column,
            };

            assembled_cells.insert(grid_position, loaded_key);
        }
        Ok(CalcKeyGrid {
            cells: assembled_cells,
        })
    }
}

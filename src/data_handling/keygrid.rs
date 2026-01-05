// build a keygrid from the RON files for a keypad

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

use crate::data_handling::grid_position::GridPosition;
use crate::data_handling::key::Error as KeyError;
use crate::data_handling::key::Key;
use crate::data_handling::key_ref::KeyRef;

//  setup error handling

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to create Key: {0}")]
    FailedKey(#[from] KeyError),
}

//  now assemble all the keys for a keypad into a grid
// ysing a hashmap with a tuple as key so i can referencw
// individual cells - the grid will render out a collection
// row widgets that will display actal buttons in a grid on screen

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct KeyGrid {
    pub cells: HashMap<GridPosition, Key>,
}

impl KeyGrid {
    /*    pub fn new(keyref_list: &Vec<CalcKeyRef>) -> Result<Self, CalcKeyGridError> {
        let mut assembled_cells = HashMap::new();

        for key_ref in keyref_list {
            let loaded_key = CalcKey::from_ron(&key_ref)?;

            let grid_position =
                CalcGridPosition::new(key_ref.row as usize, key_ref.column as usize);

            assembled_cells.insert(grid_position, loaded_key);
        }
        Ok(CalcKeyGrid {
            cells: assembled_cells,
        })
    } */
    pub fn new(keyref_list: &[KeyRef]) -> Result<Self, Error> {
        #[rustfmt::skip]
        let cells = keyref_list
            .iter()
            .map(|key_ref| {
                // Transform each key_ref into a Result containing our (Key, Value) tuple
                let loaded_key = Key::from_ron(key_ref)?;
                let grid_position = GridPosition::new(
                    key_ref.row as usize,
                    key_ref.column as usize
                );
                Ok((grid_position, loaded_key))
            })
            .collect::<Result<_, Error>>()?;

        Ok(KeyGrid { cells })
    }
}

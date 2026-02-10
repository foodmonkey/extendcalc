// build a keygrid from the RON files for a keypad

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::data::DataError;
use crate::data::Key;
use crate::data::KeyRef;
use crate::ui::GridPosition;

//  now assemble all the keys for a keypad into a grid
// ysing a hashmap with a tuple as key so i can referencw
// individual cells - the grid will render out a collection
// row widgets that will display actal buttons in a grid on screen

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct KeyGrid {
    pub cells: HashMap<GridPosition, Key>,
}

impl KeyGrid {
    pub fn new(keyref_list: &Vec<KeyRef>) -> Result<Self, DataError> {
        #[rustfmt::skip]
        let cells = keyref_list
            .iter()
            .map(|key_ref| {
                // Transform each key_ref into a Result containing our (Key, Value) tuple
                let loaded_key = Key::from_ron(key_ref)?;
                Ok((key_ref.grid_position, loaded_key))
            })
            .collect::<Result<_, DataError>>()?;

        Ok(KeyGrid { cells })
    }

    pub fn get(&self, position: &GridPosition) -> Key {
        self.cells.get(position).unwrap().clone()
    }
}

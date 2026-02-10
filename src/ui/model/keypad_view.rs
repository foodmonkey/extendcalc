// this is the collapsed keypad view - we collapse from 3NF many to many
// to a specific one to many instance of a keypad

use serde::{Deserialize, Serialize};

use crate::data::DataError;
use crate::data::Keypad;
use crate::data::KeypadRef;
use crate::ui::KeyGrid;

//  bow read our Keypads structure from the RON file

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct KeypadView {
    pub id: String,
    pub label: String,
    pub tooltip: String,
    pub rows: usize,
    pub columns: usize,
    pub keygrid: KeyGrid,
}

impl KeypadView {
    pub fn from_ron(keypad_ref: &KeypadRef) -> Result<Self, DataError> {
        let keypad = Keypad::from_ron(keypad_ref)?;
        let keygrid = KeyGrid::new(&keypad.keys)?;
        Ok(Self {
            id: keypad.id,
            label: keypad.label,
            tooltip: keypad.tooltip,
            rows: keypad.rows,
            columns: keypad.columns,
            keygrid,
        })
    }
}

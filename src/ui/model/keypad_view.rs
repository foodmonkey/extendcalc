// this is the collapsed keypad view - we collapse from 3NF many to many
// to a specific one to many instance of a keypad

use serde::{Deserialize, Serialize};

use crate::data::KeyRef;
use crate::data::Keypad;
use crate::ui::KeypadId;

//  bow read our Keypads structure from the RON file

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct KeypadView {
    pub id: KeypadId,
    pub label: String,
    pub tooltip: String,
    pub rows: usize,
    pub columns: usize,
    pub keys: Vec<KeyRef>,
}

impl From<Keypad> for KeypadView {
    fn from(keypad: Keypad) -> Self {
        Self {
            id: keypad.id.into(),
            label: keypad.label,
            tooltip: keypad.tooltip,
            rows: keypad.rows,
            columns: keypad.columns,
            keys: keypad.keys,
        }
    }
}

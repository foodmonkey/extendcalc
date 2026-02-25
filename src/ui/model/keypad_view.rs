// this is the collapsed keypad view - we collapse from 3NF many to many
// to a specific one to many instance of a keypad

use serde::{Deserialize, Serialize};

use crate::data::KeyRef;
use crate::data::KeypadRef;

//  bow read our Keypads structure from the RON file

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct KeypadView {
    pub keypad_ref: KeypadRef,
    pub label: String,
    pub tooltip: String,
    pub rows: usize,
    pub columns: usize,
    pub key_refs: Vec<KeyRef>,
}

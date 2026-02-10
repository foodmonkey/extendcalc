// build a keypad from the RON file

use serde::{Deserialize, Serialize};

use crate::data::DataDir;
use crate::data::DataError;
use crate::data::KeyRef;
use crate::data::KeypadRef;
use crate::data::helpers::load_and_parse;
use crate::data::helpers::path_builder;

//  bow read our Keypads structure from the RON file

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct Keypad {
    pub id: String,
    pub label: String,
    pub tooltip: String,
    pub rows: usize,
    pub columns: usize,
    pub keys: Vec<KeyRef>,
}

impl Keypad {
    pub fn from_ron(keypad_ref: &KeypadRef) -> Result<Self, DataError> {
        let keypad_path = path_builder(DataDir::KeypadDefinitions.as_str(), "", &keypad_ref.id);
        load_and_parse::<Self>(&keypad_path)
    }
}

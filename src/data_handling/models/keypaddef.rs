// KeypadDef - a defintion of the Keypad attributes
// also holds a collection of KeyRef to define which keys are
// included in this Keypad
use serde::{Deserialize, Serialize};

//  deserialize the keypad structure
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KeypadDef {
    pub id: String,
    pub label: String,
    pub tooltip: String,
    pub rows: u32,
    pub columns: u32,
    pub keys: Vec<KeyRef>,
}

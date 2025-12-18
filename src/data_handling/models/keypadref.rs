// KeypadRef - a key into the KeypadDef
use serde::{Deserialize, Serialize};

//  deserialize the keypad reference structure
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KeypadRef {
    pub id: String,
}

// KeypadRef - a key into the Keypad collection
use serde::{Deserialize, Serialize};

//  deserialize the keypad reference structure
#[derive(Debug, Default, Serialize, Deserialize, Hash, Eq, PartialEq, Clone)]
pub struct KeypadRef {
    pub id: String,
}

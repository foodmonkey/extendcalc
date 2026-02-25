// the reference for the lookup into the flattened 3NF structure - this
// means there is a key O(1) lookup into the flattened 3NF structure

use crate::data::KeyRef;
use crate::data::KeypadRef;

#[derive(Debug, Default, Serialize, Deserialize, Hash, Eq, PartialEq, Clone)]
pub struct KeyGridRef {
    pub keypad_ref: KeypadRef,
    pub key_ref: KeyRef,
}

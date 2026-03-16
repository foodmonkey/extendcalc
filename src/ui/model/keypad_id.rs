// this is the unique indentifier for s apecifc keypad
use indexmap::Equivalent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Hash, Eq, PartialEq, Clone)]
pub struct KeypadId {
    pub id: String,
}

// a reference KeypadId - use this to lookup in IndexMap
// without a clone on the String every time
#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub struct KeypadIdRef<'a> {
    pub id: &'a str,
}

impl KeypadId {
    pub fn new(id: String) -> Self {
        KeypadId { id }
    }
}

impl From<String> for KeypadId {
    fn from(id: String) -> Self {
        KeypadId { id }
    }
}

// and this trait tells IndexMap that it can lookup
// using a reference instead of an owned string so no cloning
impl<'a> Equivalent<KeypadId> for KeypadIdRef<'a> {
    fn equivalent(&self, key: &KeypadId) -> bool {
        self.id == key.id
    }
}

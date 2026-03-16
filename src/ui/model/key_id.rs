// this is the unique indentifier for s apecifc key in a library
// many keys can be defined in a library and shared between keypads

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Hash, Eq, PartialEq, Clone)]
pub struct KeyId {
    pub library: String,
    pub id: String,
}

impl KeyId {
    // Into<String>
    pub fn new(library: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            library: library.into(),
            id: id.into(),
        }
    }
}

// implement display for KeyId
impl std::fmt::Display for KeyId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.library, self.id)
    }
}

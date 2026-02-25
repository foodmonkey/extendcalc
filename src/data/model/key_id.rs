// this is the unique indentifier for s apecifc key in a library
// many keys can be defined in a library and shared between keypads

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Hash, Eq, PartialEq, Clone)]
pub struct KeyId {
    pub library: String,
    pub id: String,
}

impl KeyId {
    pub fn new(library: String, id: String) -> Self {
        KeyId { library, id }
    }
}

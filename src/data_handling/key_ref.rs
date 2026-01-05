// Key - a key into the KeydDef
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone, Ord, PartialOrd, Eq, PartialEq)]

pub struct KeyRef {
    pub library: String,
    pub id: String,
    pub row: u32,
    pub column: u32,
}

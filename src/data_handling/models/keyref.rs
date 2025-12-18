// Key - a key into the KeydDef
use serde::{Deserialize, Serialize};

//  deserialize the key definition structure
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KeyRef {
    pub library: String,
    pub id: String,
}

// Keys a collection of KeyRed (a key to the Key model)
use serde::{Deserialize, Serialize};

// deserialize the collection of keypads
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Keys {
    pub keys: Vec<KeyRef>,
}

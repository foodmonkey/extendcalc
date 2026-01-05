// Keys a collection of CalcKeyRef which allows us to list
// all the keys available in all the libraries
use crate::data_handling::key_ref::KeyRef;
use serde::{Deserialize, Serialize};

// deserialize the collection of keypads
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KeyList {
    pub keysref: Vec<KeyRef>,
}

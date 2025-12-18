// Keypads a collection of KeypadRef (a key to the Keypad model)
use serde::{Deserialize, Serialize};

// deserialize the collection of keypads
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Keypads {
    pub keypads: Vec<KeypadRef>,
}

// KeyDef - definition of the Key structire
use serde::{Deserialize, Serialize};

//  desrialize the key definition structure
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KeyDef {
    pub id: String,
    pub label: String,
    pub tooltip: String,
    pub qalc_term: String,
    pub row: u32,
    pub column: u32,
}

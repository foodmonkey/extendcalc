// panelRef - a key into the Panel library
use serde::{Deserialize, Serialize};

//  deserialize the keypad reference structure
#[derive(Debug, Default, Deserialize, Serialize, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PanelRef {
    pub id: String,
}

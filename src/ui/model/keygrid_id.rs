// the reference for the lookup into the flattened 3NF structure - this
// means there is a key O(1) lookup into the flattened 3NF structure

use crate::ui::GridPosition;
use crate::ui::KeypadId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Hash, Eq, PartialEq, Clone)]
pub struct KeyGridId {
    pub keypad_id: KeypadId,
    pub grid_position: GridPosition,
}

// KeyRef - lets link the 3NF stuff in the RON dirs with the View stuff
//
use crate::data::KeyId;
use crate::ui::GridPosition;
use crate::ui::GridSpan;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct KeyRef {
    pub key_id: KeyId,
    pub grid_position: GridPosition,
    #[serde(default)]
    pub grid_span: GridSpan,
}

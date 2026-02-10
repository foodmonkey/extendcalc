// Key - a key into the KeydDef
use crate::ui::GridPosition;
use crate::ui::GridSpan;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct KeyRef {
    pub library: String,
    pub id: String,
    pub grid_position: GridPosition,
    #[serde(default)]
    pub grid_span: GridSpan,
}

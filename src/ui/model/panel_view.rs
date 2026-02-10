// this is the panel view - it represents the structure
// in the RON files - we have this because of data and
// view separation

use serde::{Deserialize, Serialize};

use crate::data::DataError;
use crate::data::KeypadRef;
use crate::data::Panel;
use crate::data::PanelRef;

//  bow read our Keypads structure from the RON file

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct PanelView {
    pub id: String,
    pub label: String,
    pub tooltip: String,
    pub keypads: Vec<KeypadRef>,
}

impl PanelView {
    pub fn from_ron(panel_ref: &PanelRef) -> Result<Self, DataError> {
        let panel = Panel::from_ron(panel_ref)?;
        Ok(Self {
            id: panel.id,
            label: panel.label,
            tooltip: panel.tooltip,
            keypads: panel.keypads,
        })
    }
}

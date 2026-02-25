// build a panel from the RON file

use serde::{Deserialize, Serialize};

use crate::data::DataDir;
use crate::data::DataError;
use crate::data::KeypadRef;
use crate::data::PanelRef;
use crate::data::helper::load_and_parse;
use crate::data::helper::path_builder;

//  bow read our Panel structure from the RON file

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct Panel {
    pub id: String,
    pub label: String,
    pub tooltip_text: String,
    pub rows: usize,
    pub columns: usize,
    pub keypads: Vec<KeypadRef>,
}

impl Panel {
    pub fn from_ron(panel_ref: &PanelRef) -> Result<Self, DataError> {
        let panel_path = path_builder(DataDir::Data, DataDir::PanelDefinitions, "", &panel_ref.id);
        load_and_parse::<Self>(&panel_path)
    }
}

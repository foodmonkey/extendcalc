// build a list of panels from the RON file

use serde::{Deserialize, Serialize};

use crate::data::DataDir;
use crate::data::DataError;
use crate::data::PanelRef;
use crate::data::helpers::load_and_parse;
use crate::data::helpers::path_builder;

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct PanelList {
    pub panel_refs: Vec<PanelRef>,
}

//  bow read our Keypads structure from the RON file

impl PanelList {
    pub fn from_ron() -> Result<Self, DataError> {
        println!("Loading panellist");
        let list_path = path_builder(DataDir::PanelDefinitions.as_str(), "", "panel_list");
        load_and_parse::<Self>(&list_path)
    }
}

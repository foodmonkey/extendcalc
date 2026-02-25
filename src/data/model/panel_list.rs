// build a list of panels from the RON file

use serde::{Deserialize, Serialize};

use crate::data::DataDir;
use crate::data::DataError;
use crate::data::PanelRef;
use crate::data::helper::load_and_parse;
use crate::data::helper::path_builder;

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct PanelList {
    pub panel_refs: Vec<PanelRef>,
}

//  bow read our Keypads structure from the RON file

impl PanelList {
    pub fn from_ron() -> Result<Self, DataError> {
        let list_path = path_builder(
            DataDir::Data,
            DataDir::PanelDefinitions.as_str(),
            "",
            "panel_list",
        );
        load_and_parse::<Self>(&list_path)
    }

    pub fn push(&mut self, panel_ref: PanelRef) {
        self.panel_refs.push(panel_ref);
    }

    pub fn len(&self) -> usize {
        self.panel_refs.len()
    }
}

impl<'a> IntoIterator for &'a PanelList {
    type Item = &'a PanelRef;
    type IntoIter = std::slice::Iter<'a, PanelRef>;

    fn into_iter(self) -> Self::IntoIter {
        self.panel_refs.iter()
    }
}

impl<'a> IntoIterator for &'a mut PanelList {
    type Item = &'a mut PanelRef;
    type IntoIter = std::slice::IterMut<'a, PanelRef>;

    fn into_iter(self) -> Self::IntoIter {
        self.panel_refs.iter_mut()
    }
}

impl IntoIterator for PanelList {
    type Item = PanelRef;
    type IntoIter = std::vec::IntoIter<PanelRef>;

    fn into_iter(self) -> Self::IntoIter {
        self.panel_refs.into_iter()
    }
}

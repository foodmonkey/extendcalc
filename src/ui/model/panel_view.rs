// this is the panel view - it is the skinny structure for
// the view - this separaters UI from the RON data

use crate::data::KeypadRef;
use crate::data::Panel;

//  bow read our Keypads structure from the RON file

#[derive(Debug, Default, Clone)]
pub struct PanelView {
    pub id: String,
    pub label: String,
    pub tooltip_text: String,
    pub rows: usize,
    pub columns: usize,
    pub keypads: Vec<KeypadRef>,
}

impl From<Panel> for PanelView {
    fn from(panel: Panel) -> Self {
        Self {
            id: panel.id,
            label: panel.label,
            tooltip_text: panel.tooltip_text,
            rows: panel.rows,
            columns: panel.columns,
            keypads: panel.keypads,
        }
    }
}

impl<'a> IntoIterator for &'a PanelView {
    // What the loop yields: a reference to a KeypadRef
    type Item = &'a KeypadRef;

    // The engine: we borrow the one already built into Vec
    type IntoIter = std::slice::Iter<'a, KeypadRef>;

    fn into_iter(self) -> Self::IntoIter {
        self.keypads.iter()
    }
}

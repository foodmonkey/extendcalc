// the collection of panels - we use this so we can implement len
// and a "get" on a specific panel id when we switch views
// in the navbar
use crate::ui::PanelView;

#[derive(Debug, Clone)]
pub struct Panels {
    panel_views: Vec<PanelView>,
    to_load: usize,
}

impl Panels {
    pub fn new() -> Self {
        Panels {
            panel_views: Vec::new(),
            to_load: 0,
        }
    }

    pub fn get(&self, id: &str) -> &PanelView {
        self.panel_views
            .iter()
            .find(|panel_view| panel_view.id == id)
            .unwrap()
    }

    pub fn insert(&mut self, panel_view: PanelView) {
        self.panel_views.push(panel_view);
        self.to_load += 1;
    }

    pub fn len(&self) -> usize {
        self.panel_views.len()
    }
}

impl<'a> IntoIterator for &'a Panels {
    // What the loop yields: a reference to a Panel
    type Item = &'a PanelView;

    // The engine: we borrow the one already built into Vec
    type IntoIter = std::slice::Iter<'a, PanelView>;

    fn into_iter(self) -> Self::IntoIter {
        self.panel_views.iter()
    }
}

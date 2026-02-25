// the collection of panels - we use this so we can implement len
// and a "get" on a specific panel id when we switch views
// in the navbar

use crate::ui::AsyncCountdown;
use crate::ui::PanelView;

#[derive(Debug, Default, Clone)]
pub struct Panels {
    panel_views: Vec<PanelView>,
    countdown: AsyncCountdown,
}

impl Panels {
    pub fn get(&self, panel_id: &str) -> &PanelView {
        self.panel_views
            .iter()
            .find(|pamel| panel.id == panel_id)
            .unwrap()
    }

    pub fn push(&mut self, panel_view: PanelView) {
        self.panel_views.push(panel_view);
    }

    pub fn len(&self) -> usize {
        self.panel_views.len()
    }

    pub fn track_async(&mut self, count: usize) {
        self.countdown.track(count);
    }

    pub fn async_remaining(&self) -> usize {
        self.countdown.remaining()
    }

    pub fn async_finished(&self) -> bool {
        self.countdown.is_zero()
    }
}

impl<'a> IntoIterator for &'a Panels {
    type Item = &'a PanelView;
    type IntoIter = std::slice::Iter<'a, PanelView>;

    fn into_iter(self) -> Self::IntoIter {
        self.panel_views.iter()
    }
}

impl<'a> IntoIterator for &'a mut Panels {
    type Item = &'a mut PanelView;
    type IntoIter = std::slice::IterMut<'a, PanelView>;

    fn into_iter(self) -> Self::IntoIter {
        self.panel_views.iter_mut()
    }
}

impl IntoIterator for Panels {
    type Item = PanelView;
    type IntoIter = std::vec::IntoIter<PanelView>;

    fn into_iter(self) -> Self::IntoIter {
        self.panel_views.into_iter()
    }
}

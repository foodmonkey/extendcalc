// ok here's where we start building dipslay widgets to represent
// the PnaelView - which will contain 1 or more KeypadViews

use cosmic::Element;
use cosmic::widget::{container, row};

use crate::app::UiModel;

use crate::app::Message;

impl UiModel {
    pub fn render_panel_view(&self) -> Element<'static, Message> {
        let active_panel_id = self.navbar.active_data::<String>().unwrap();
        let active_panel = self.panels.get(&active_panel_id);

        let mut panel_row = row::with_capacity(active_panel.keypads.len());

        for keypad_ref in active_panel {
            panel_row = panel_row.push(self.render_keypad_view(&keypad_ref));
        }
        container(panel_row).into()
    }
}

// render the KeypadView and return a container

use cosmic::Element;

use crate::app::Message;
use crate::app::UiModel;
use crate::data::KeypadRef;

use crate::ui::helper::build_button_grid;

impl UiModel {
    pub fn render_keypad_view(&self, keypad_ref: &KeypadRef) -> Element<'static, Message> {
        let active_keypad_view = self.keypads.get(&keypad_ref);

        build_button_grid(
            &active_keypad_view.keygrid,
            active_keypad_view.rows,
            active_keypad_view.columns,
        )
    }
}

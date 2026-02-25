// handler for panels loaded - we build a list of keypads to be loaded
// and pass it to the LoadKeypads message

use cosmic::app::Task;

use crate::app::Message;
use crate::app::UiModel;

use crate::data::KeypadList;

impl UiModel {
    pub fn panels_loaded(&mut self) -> Task<Message> {
        let mut keypad_list = KeypadList::default();

        for panel_view in &self.panels {
            for keypad_ref in panel_view {
                if !keypad_list.contains(&keypad_ref) {
                    keypad_list.push(keypad_ref.clone());
                }
            }
        }

        Task::done(cosmic::action::app(Message::LoadKeypads(keypad_list)))
    }
}

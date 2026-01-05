// let's process the messages outstanding for the
// AssembledKeypads model

use crate::ui::assembled_keypads::model::AssembledKeypads;
use crate::ui::assembled_keypads::model::Message;
use cosmic::Task;

impl AssembledKeypads {
    // process the update part of the MVU loop for
    // assembledKeypads

    pub fn update(&mut self, message: Message) -> Task<cosmic::Action<Message>> {
        match message {
            Message::Initialise => Task::none(),
            Message::Keypad(keypad_action) => self.handle_keypad_action(keypad_action),
            Message::List(list_action) => self.handle_list_action(list_action),
        }
    }
}

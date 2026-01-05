// Handles messages emitted by the application and its widgets.
//
// Tasks may be returned for asynchronous execution of code in the background
// on the application's async runtime.

use super::model::AppModel;
use super::model::Message;

use cosmic::Task;

impl AppModel {
    pub(super) fn update(&mut self, message: Message) -> Task<cosmic::Action<Message>> {
        match message {
            Message::AssembledKeypads(inner_msg) => self
                .assembled_keypads
                .update(inner_msg)
                //               .map(|action| action.map(Message::AssembledKeypads)),
                .map(|action| action.map(Message::AssembledKeypads)),
        }
    }
}

// handler for panels loaded - we spawn the batch of keypad load
// tasks - fully transient because we don't have to look up an RON`

use cosmic::app::Task;

use crate::app::Message;
use crate::app::UiModel;

use crate::data::KeypadList;

impl UiModel {
    pub fn load_keypads(&self, keypad_list: KeypadList) -> Task<Message> {
        let mut task_batch = Vec::new();
        let count = keypad_list.len();

        for keypad_ref in keypad_list {
            let keypad_ref_owned = keypad_ref.clone();
            task_batch.push(Task::future(async move {
                cosmic::action::app(Message::LoadKeypad(keypad_ref_owned, count))
            }));
        }

        Task::batch(task_batch)
    }
}

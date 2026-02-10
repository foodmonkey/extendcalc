// generate the bacth of tasks that will asynx load the keypad models

use cosmic::app::Task;

use crate::app::Message;
use crate::app::UiModel;
use crate::data::KeypadRef;

pub fn generate_keypadview_batch(ui: &UiModel) -> Task<Message> {
    let mut keypad_list: Vec<KeypadRef> = Vec::new();
    for panel in &ui.panels {
        for keypad_ref in &panel.keypads {
            if !keypad_list.contains(&keypad_ref) {
                keypad_list.push(keypad_ref.clone());
            }
        }
    }

    let mut task_batch = Vec::new();

    for keypad_ref in keypad_list {
        let keypad_ref_owned = keypad_ref.clone();
        task_batch.push(Task::future(async move {
            cosmic::action::app(Message::LoadKeypad(keypad_ref_owned))
        }));
    }

    Task::batch(task_batch)
}

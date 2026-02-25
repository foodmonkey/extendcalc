// load the keypad from the RON file
use crate::app::Message;
use crate::data::Keypad;
use crate::data::KeypadRef;
use cosmic::app::Task;

pub fn load_keypad(keypad_ref: KeypadRef, count: usize) -> Task<Message> {
    // load the keypad and create the future panel loaded task
    Task::future(async move {
        let result =
            Keypad::from_ron(&keypad_ref).map_err(|e| format!("fail keypad load: {:?}", e));
        cosmic::action::app(Message::KeypadLoaded(result, count))
    })
}

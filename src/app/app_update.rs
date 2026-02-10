// Handles messages emitted by the application and its widgets.
// Tasks may be returned for asynchronous execution of code in the background
// on the application's async runtime.
use cosmic::app::Task;

use crate::app::AppModel;
use crate::app::AppState;
use crate::app::Message;

impl AppModel {
    pub fn app_update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::LoadPanelList => Task::none(),
            Message::PanelListLoaded(result) => Task::none(),
            Message::LoadPanel(panel_ref) => Task::none(),
            Message::PanelLoaded(result) => Task::none(),
            Message::GenerateKeypadLoadBatch => Task::none(),
            Message::LoadKeypad(keypad_ref) => Task::none(),
            Message::KeypadLoaded(result) => Task::none(),
            Message::ChangePanel(id) => {
                println!("msg ChangeKeypad");
                self.ui.navbar.activate(id);
                Task::none()
            }

            Message::KeyPressed(key) => {
                println!("msg KeyPressed {}", key);
                Task::none()
            }

            Message::Error(error) => {
                self.state = AppState::Error(error);
                Task::none()
            }
        }
    }
}

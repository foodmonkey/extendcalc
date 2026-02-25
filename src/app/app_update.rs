// Handles messages emitted by the application and its widgets.
// Tasks may be returned for asynchronous execution of code in the background
// on the application's async runtime.
use cosmic::app::Task;

use crate::app::AppModel;
use crate::app::AppState;
use crate::app::Message;

use crate::data::helper as data_helper;

impl AppModel {
    pub fn app_update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::LoadPanelList => data_helper::load_panel_list(),
            Message::PanelListLoaded(result) => self.ui.panel_list_loaded(result),

            Message::LoadPanels(panel_list) => data_helper::load_panels(panel_list),
            Message::LoadPanel(panel_ref, count) => data_helper::load_panel(panel_ref, count),
            Message::PanelLoaded(result, count) => self.ui.panel_loaded(result, count),
            Message::PanelsLoaded => self.ui.panels_loaded(),

            Message::LoadKeypads(keypad_list) => data_helper::load_keypads(keypad_list),
            Message::LoadKeypad(keypad_ref, count) => data_helper::load_keypad(keypad_ref, count),
            Message::KeypadLoaded(result, count) => self.ui.keypad_loaded(result, count),
            Message::KeypadsLoaded => self.ui.keypads_loaded(),

            Message::GenerateSvg(key_identity, render_string, count) => Task::none(),
            Message::SvgGenerated(count) => Task::none(),
            Message::ChangeAppState(state) => {
                self.state = state;
                Task::none()
            }

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

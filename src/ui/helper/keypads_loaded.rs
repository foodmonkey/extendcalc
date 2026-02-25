// handler for keypads loaded
use cosmic::app::Task;

use crate::app::AppState;
use crate::app::InitState;
use crate::app::Message;
use crate::app::UiModel;

impl UiModel {
    pub fn keypads_loaded(&mut self) -> Task<Message> {
        Task::done(cosmic::action::app(Message::ChangeAppState(
            AppState::Init(InitState::Loaded),
        )))
    }
}

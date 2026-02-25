// handler for panellist loaded - we check the result and pass the
// panel list to the LoadPanels helper

use crate::app::Message;
use crate::app::UiModel;
use crate::data::PanelList;
use cosmic::app::Task;

impl UiModel {
    pub fn panel_list_loaded(&self, result: Result<PanelList, String>) -> Task<Message> {
        match result {
            Ok(panel_list) => Task::done(cosmic::action::app(Message::LoadPanels(panel_list))),

            Err(error) => Task::done(cosmic::action::app(Message::Error(error))),
        }
    }
}

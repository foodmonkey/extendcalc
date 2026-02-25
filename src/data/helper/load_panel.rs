// load the panel from the RON file
use crate::app::Message;
use crate::data::Panel;
use crate::data::PanelRef;
use cosmic::app::Task;

pub fn load_panel(panel_ref: PanelRef, count: usize) -> Task<Message> {
    // load the panel and create the future panel loaded task
    Task::future(async move {
        let result = Panel::from_ron(&panel_ref).map_err(|e| format!("fail panel load: {:?}", e));
        cosmic::action::app(Message::PanelLoaded(result, count))
    })
}

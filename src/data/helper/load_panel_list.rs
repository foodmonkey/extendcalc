// load the PanelList from the RON file
use crate::app::Message;
use crate::data::PanelList;
use cosmic::app::Task;

pub fn load_panel_list() -> Task<Message> {
    // load the keypad and create the future panel loaded task
    Task::future(async move {
        let result = PanelList::from_ron().map_err(|e| format!("fail panellist load: {:?}", e));
        cosmic::action::app(Message::PanelListLoaded(result))
    })
}

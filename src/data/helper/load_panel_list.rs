// load the panel list from the RON file
use crate::app::Message;
use crate::data::PanelList;
use cosmic::app::Task;

pub fn load_panel_list() -> Task<Message> {
    println!("load panel list");
    // create the task to load the panel list
    Task::future(async move {
        let result = PanelList::from_ron().map_err(|e| format!("fail panellist load: {:?}", e));
        cosmic::action::app(Message::PanelListLoaded(result))
    })
}

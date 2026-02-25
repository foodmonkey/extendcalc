// helper to load the panels - get the panel list from the RON
// loop through them and generate the batch of async tasks to
// load each panel

use cosmic::app::Task;

use crate::app::Message;
use crate::data::PanelList;

pub fn load_panels(panel_list: PanelList) -> Task<Message> {
    let mut task_batch = Vec::new();
    let async_count = panel_list.len();

    for panel_ref in panel_list {
        task_batch.push(Task::future(async move {
            cosmic::action::app(Message::LoadPanel(panel_ref, async_count))
        }));
    }

    Task::batch(task_batch)
}

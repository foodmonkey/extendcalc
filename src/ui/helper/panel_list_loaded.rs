// handler for panel list loaded - just shift the result
// into the panel list or change state to error and issue
// error message - if ok then generate a batch of async
// load panel tasks

use crate::app::Message;
use crate::app::UiModel;
use crate::data::PanelList;
use cosmic::app::Task;

pub fn panel_list_loaded(ui: &mut UiModel, result: Result<PanelList, String>) -> Task<Message> {
    let mut task_batch = Vec::new();

    match result {
        Ok(list) => {
            ui.panel_list = list;

            for panel_ref in &ui.panel_list.panel_refs {
                let panel_ref_owned = panel_ref.clone();
                task_batch.push(Task::future(async move {
                    cosmic::action::app(Message::LoadPanel(panel_ref_owned))
                }));
            }
            Task::batch(task_batch)
        }
        Err(error) => Task::done(cosmic::action::app(Message::Error(error))),
    }
}

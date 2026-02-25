// handler for panel loaded - just shift the result
// into the panel list and increment the loaded count

use cosmic::app::Task;

use crate::app::Message;
use crate::app::UiModel;
use crate::data::Panel;
use crate::ui::PanelView;

impl UiModel {
    pub fn panel_loaded(&mut self, result: Result<Panel, String>, count: usize) -> Task<Message> {
        self.panels.track_async(count);

        let mut tasks_batch = Vec::new();
        match result {
            Ok(panel) => {
                let panel_view = PanelView::from(panel);

                self.panels.push(panel_view.clone());

                match self.panels.len() {
                    1 => self
                        .navbar
                        .insert()
                        .data(panel_view.id.clone())
                        .text(panel_view.label.clone())
                        .activate(),
                    _ => self
                        .navbar
                        .insert()
                        .data(panel_view.id.clone())
                        .text(panel_view.label.clone()),
                };

                tasks_batch.push(Task::none());
            }

            Err(data_error) => {
                tasks_batch.push(Task::done(cosmic::action::app(Message::Error(data_error))));
            }
        }

        if self.panels.async_finished() {
            tasks_batch.push(Task::done(cosmic::action::app(Message::PanelsLoaded)));
        }

        Task::batch(tasks_batch)
    }
}

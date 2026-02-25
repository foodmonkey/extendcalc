// handler for keyoad loaded - we shift the KeypadData to the KeypadView
// and then "tick" the Keypads async count

use cosmic::app::Task;

use crate::app::Message;
use crate::app::UiModel;
use crate::data::KeypadData;
use crate::ui::KeypadView;

impl UiModel {
    pub fn keypad_loaded(
        &mut self,
        result: Result<KeypadData, String>,
        count: usize,
    ) -> Task<Message> {
        self.keypads.track_async(count);

        let mut tasks_batch = Vec::new();
        match result {
            Ok(keypad_data) => {
                let keypad_view = KeypadView::from(keypad_data);
                self.keypads.push(keypad_view);

                tasks_batch.push(Task::none());
            }

            Err(error) => {
                tasks_batch.push(Task::done(cosmic::action::app(Message::Error(error))));
            }
        }

        if self.keypads.async_finished() {
            tasks_batch.push(Task::done(cosmic::action::app(Message::KeypadsLoaded)));
        }

        Task::batch(tasks_batch)
    }
}

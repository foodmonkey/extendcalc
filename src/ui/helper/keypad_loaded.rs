// handler for keyoad loaded - we shift the KeypadData to the KeypadView
// and then "tick" the Keypads async count

use cosmic::app::Task;

use crate::app::Message;
use crate::app::UiModel;
use crate::data::Keypad;
use crate::data::KeypadRef;
use crate::ui::KeypadView;

impl UiModel {
    pub fn keypad_loaded(
        &mut self,
        result: Result<Keypad, String>,
        keypad_ref: KeypadRef,
        count: usize,
    ) -> Task<Message> {
        self.keypads.track_async(count);

        let mut tasks_batch = Vec::new();
        match result {
            Ok(keypad) => {
                let keypad_view = KeypadView::from(keypad);
                self.keypads.insert(&keypad_ref, &keypad_view);

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

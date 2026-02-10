// Describes the interface based on the current state of the application model.
// Application events will be processed through the view. Any messages emitted by
// events received by widgets will be passed to the update method.

use cosmic::Element;
use cosmic::widget::text;

use crate::app::AppState;
use crate::app::Message;
use crate::app::UiModel;
use crate::ui::helper::build_button_grid;

impl UiModel {
    pub fn app_view(&self) -> Element<'_, Message> {
        match self.state {
            AppState::Init(InitState::Loading) => text("Init Loading").into(),
            AppState::Init(InitState::Loaded) => text("Loaded").into(),
            AppState::Ready => text("Ready").into(),
            AppState::Error(error) => text(format!("Error: {}", error)).into(),
        }
        println!("in app_view");

        let model_id = self.navbar.active_data::<String>();

        let keypad_grid: Element<'_, Message> = match model_id {
            None => text("Loading Keypads").into(),
            Some(id) => {
                println!("view model_id: {}", id);
                let id_cloned = id.clone();
                let keypad_model = self
                    .keypads
                    .iter()
                    .find(|k| k.id == id_cloned)
                    .expect("keypad model not found");

                build_button_grid(
                    &keypad_model.keygrid,
                    keypad_model.rows,
                    keypad_model.columns,
                )
                .into()
            }
        };
        keypad_grid.into()
    }
}

// Describes the interface based on the current state of the application model.
// Application events will be processed through the view. Any messages emitted by
// events received by widgets will be passed to the update method.

use cosmic::Element;
use cosmic::widget::text;

use crate::app::AppState;
use crate::app::InitState;
use crate::app::Message;
use crate::app::UiModel;
//use crate::ui::helper::build_button_grid;

impl UiModel {
    pub fn view(&self, app_state: &AppState) -> Element<'_, Message> {
        use AppState::*;
        use InitState::*;

        match app_state {
            Init(Loading) => text("Init Loading").into(),
            Init(Loaded) => self.render_panel_view(),
            Ready => text("Ready").into(),
            Error(error) => text(format!("Error: {}", error)).into(),
        }
    }
}

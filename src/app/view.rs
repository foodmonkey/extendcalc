// Describes the interface based on the current state of the application model.
// Application events will be processed through the view. Any messages emitted by
// events received by widgets will be passed to the update method.

use super::model::AppModel;
use super::model::Message;
use cosmic::Element;

impl AppModel {
    pub(super) fn view(&self) -> Element<'_, Message> {
        cosmic::widget::text("Hello extendcalc").into()
    }
}

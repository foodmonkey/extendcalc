// this is the App and AppModel and implementation
//
// The application model stores app-specific state used to describe its interface and
// drive its logic.
//
use cosmic::prelude::*;
use cosmic::widget::nav_bar;

use crate::ui::assembled_keypads::model::AssembledKeypads;
use crate::ui::assembled_keypads::model::Message as KeypadsMessage;

const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
const APP_ICON: &[u8] = include_bytes!("../../resources/icons/hicolor/scalable/apps/icon.svg");

pub struct AppModel {
    core: cosmic::Core,
    navbar: nav_bar::Model,
    pub(super) assembled_keypads: AssembledKeypads,
}

// Messages emitted by the application and its widgets.
#[derive(Debug, Clone)]
pub enum Message {
    AssembledKeypads(KeypadsMessage),
}

/// Create a COSMIC application from the app model
impl cosmic::Application for AppModel {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;
    const APP_ID: &'static str = "com.github.foodmonkey.extendcalc";

    fn core(&self) -> &cosmic::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::Core {
        &mut self.core
    }

    // Initializes the application with any given flags and startup commands.
    //
    fn init(
        core: cosmic::Core,
        _flags: Self::Flags,
    ) -> (Self, Task<cosmic::Action<Self::Message>>) {
        // Construct the app model with the runtime's core.
        let app = AppModel {
            core,
            navbar: nav_bar::Model::default(),
            assembled_keypads: AssembledKeypads::default(),
        };

        // issue a message to assembled keypads to load all
        // the assembled keypad instances
        let keypads_task = Task::done(Message::AssembledKeypads(KeypadsMessage::Initialise));
        (app, keypads_task.map(cosmic::Action::App))
    }
}

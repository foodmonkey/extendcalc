use cosmic::app::{Application, Core, Task};
// use cosmic::prelude::*;
use cosmic::widget::nav_bar;

use crate::app::AppModel;
use crate::app::AppState;
use crate::app::Message;
use crate::app::UiModel;
use crate::app::app_update;
use crate::app::app_view;

impl Application for AppModel {
    type Flags = ();
    type Executor = cosmic::executor::Default;
    type Message = Message;

    const APP_ID: &'static str = "com.github.foodmonkey.extendcalc";

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Self::Message>) {
        let app = AppModel {
            core,
            state: AppState::default(),
            ui: UiModel::default(),
        };

        let startup_task = Task::future(async move { cosmic::action::app(Message::LoadPanelList) });

        (app, startup_task)
    }

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn nav_model(&self) -> Option<&nav_bar::Model> {
        Some(&self.ui.navbar)
    }

    fn on_nav_select(&mut self, id: nav_bar::Id) -> Task<Self::Message> {
        match self.ui.navbar.is_active(id) {
            true => Task::none(),
            //            false => Task::done(cosmic::action::app(Message::ChangeKeypad(id))),
            false => Task::none(),
        }
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        self.app_update(message)
    }

    fn view(&self) -> cosmic::Element<'_, Self::Message> {
        self.app_view()
    }
}

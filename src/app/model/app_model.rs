// this is the definition of the appmodel
use cosmic::Core;
use cosmic::widget::nav_bar;

use crate::app::AppState;

use crate::data::KeypadRef;
use crate::data::Panel;
use crate::data::PanelList;
use crate::data::PanelRef;

use crate::ui::KeypadView;
use crate::ui::Keypads;
use crate::ui::Panels;

#[derive(Debug, Clone)]
pub enum Message {
    LoadPanelList,
    PanelListLoaded(Result<PanelList, String>),

    LoadPanel(PanelRef),
    PanelLoaded(Result<Panel, String>),

    GenerateKeypadLoadBatch,

    LoadKeypad(KeypadRef),
    KeypadLoaded(Result<KeypadView, String>),

    ChangePanel(nav_bar::Id),
    KeyPressed(String),

    Error(String),
}

pub struct AppModel {
    pub core: Core,
    pub state: AppState,
    pub ui: UiModel,
}

pub struct UiModel {
    pub navbar: nav_bar::Model,

    pub panel_list: PanelList,
    pub panels: Panels,
    pub panels_loaded: usize,

    pub keypads: Keypads,
    pub keypads_loaded: usize,
}

impl Default for UiModel {
    fn default() -> Self {
        Self {
            navbar: nav_bar::Model::default(),
            panel_list: PanelList::default(),
            panels: Panels::new(),
            panels_loaded: 0,
            keypads: Keypads::new(),
            keypads_loaded: 0,
        }
    }
}

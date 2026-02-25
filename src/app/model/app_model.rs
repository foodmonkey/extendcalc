// this is the definition of the appmodel
use cosmic::Core;
use cosmic::widget::nav_bar;

use crate::app::AppState;

use crate::data::KeyId;
use crate::data::Keypad;
use crate::data::KeypadList;
use crate::data::KeypadRef;
use crate::data::Panel;
use crate::data::PanelList;
use crate::data::PanelRef;

use crate::ui::KeySvg;
use crate::ui::Keypads;
use crate::ui::Panels;

#[derive(Debug, Clone)]
pub enum Message {
    LoadPanelList,
    PanelListLoaded(Result<PanelList, String>),

    LoadPanels(PanelList),
    LoadPanel(PanelRef, usize),
    PanelLoaded(Result<Panel, String>, usize),
    PanelsLoaded,

    LoadKeypads(KeypadList),
    LoadKeypad(KeypadRef, usize),
    KeypadLoaded(Result<Keypad, String>, usize),
    KeypadsLoaded,

    LoadKeyGrids,
    LoadKey(usize),
    KeyLoaded(Result<KeyId, String>, usize),
    GenerateSvg(usize),
    SvgGenerated(usize),
    KeyGridsLoaded,
    SVGsLoaded,

    ChangeAppState(AppState),
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
    pub panels: Panels,
    pub keypads: Keypads,
    pub key_svg: KeySvg,
}

impl Default for UiModel {
    fn default() -> Self {
        Self {
            navbar: nav_bar::Model::default(),
            panels: Panels::default(),
            keypads: Keypads::default(),
            key_svg: KeySvg::new(),
        }
    }
}

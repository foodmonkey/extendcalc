// this is the definition of the appmodel
use cosmic::Core;
use cosmic::widget::nav_bar;

use crate::app::AppState;

use crate::data::Key;
use crate::data::KeyRef;
use crate::data::Keypad;
use crate::data::KeypadList;
use crate::data::KeypadRef;
use crate::data::Panel;
use crate::data::PanelList;
use crate::data::PanelRef;

use crate::ui::KeyGrids;
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
    KeypadLoaded(Result<Keypad, String>, KeypadRef, usize),
    KeypadsLoaded,

    LoadKeyGrids,
    LoadKey(KeyRef, usize),
    KeyLoaded(Result<Key, String>, KeyGridRef, usize),
    GenerateSvg(KeyId, String, usize),
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
    pub keygrids: KeyGrids,
    pub key_svg: KeySvg,
}

impl Default for UiModel {
    fn default() -> Self {
        Self {
            navbar: nav_bar::Model::default(),
            panels: Panels::default(),
            keypads: Keypads::default(),
            keygrids: KeyGrids::default(),
            key_svg: KeySvg::new(),
        }
    }
}

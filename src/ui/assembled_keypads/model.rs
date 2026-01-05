//  init error handling from the reading of the RON and
// building the Calc structures

use serde::{Deserialize, Serialize};

use crate::data_handling::grid_position::GridPosition;
use crate::data_handling::keypad_ref::KeypadRef;
use crate::data_handling::keypads::Keypads;
use crate::ui::assembled_keypad::AssembledKeypad;

//  assembled keypads state
#[derive(Debug, Default, Clone)]
pub enum State {
    #[default]
    Assembling,
    Assembled,
    Error(String),
}

// assembled keypad set are we grabbing
// UI - the subset to build the display
// Library - the set of ALL keypads
#[derive(Debug, Default, Clone)]
pub enum KeypadSet {
    #[default]
    UI,
    All,
}

// messages for assembledkeypads
// the stuff that can happen to the keypad list
#[derive(Debug, Clone)]
pub enum ListAction {
    Load(KeypadSet, String),
    LoadedOk,
    LoadFailed,
    SwitchTo(String),
}

// the stuff that can happen to the keypad
#[derive(Debug, Clone)]
pub enum KeypadAction {
    Assemble(KeypadRef),
    AssembledOK,
    AssembleFailed,
    KeyPressed(GridPosition, String),
}

// the central router for messages for the keypads
#[derive(Debug, Clone)]
pub enum Message {
    Initialise,
    List(ListAction),
    Keypad(KeypadAction),
}

//  AssembledKeypads
#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct AssembledKeypads {
    pub keypads_list: Keypads,
    pub keypads: Vec<AssembledKeypad>,
    #[serde(skip)]
    pub keypads_state: State,
    pub keypads_to_load: u32,
    pub active_keypad_id: String,
}

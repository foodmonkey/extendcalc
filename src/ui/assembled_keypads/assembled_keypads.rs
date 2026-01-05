// struct to hold all the keypad tree
// we build this from the json files and then use it
// to construct the ui - it gets saved in the config info
// so we don't have to read the JSON files every time at startup

use cosmic::{Element, Task};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::data_handling::assembled_keypad::{AssembledKeypad, AssembledKeypadError};
use crate::data_handling::calc_gridposition::CalcGridPosition;
use crate::data_handling::calc_keypads::{CalcKeypads, CalcKeypadsError};
use crate::globals::data_constants::{KEYPADS_PATH, KEYPADS_UI_RON};

// now we read the list of keypad refs from the keypads JSON file

impl AssembledKeypads {
    pub fn new() -> (Self, Task<AssembledKeypadsMessage>) {
        (
            Self {
                keypads_list: CalcKeypads::default(),
                keypads: Vec::new(),
                keypads_state: AssembledKeypadState::Loading,
                active_keypad_id: usize::default(),
            },
            // perform an async task to load the keypads so that
            // the app is not blocked
            Task::perform(
                Self::load_assembled_keypads(),
                AssembledKeypadsMessage::Loaded,
            ),
        )
    }

    // this is the async task to build each assembled keypad and push
    // it into the keypads collection and return that collection when
    // finished and change our state to loaded
    async fn load_assembled_keypads() -> Result<AssembledKeypads, AssembledKeypadsError> {
        //  load the list of keypads for the UI
        let keypads_list_path = format!("{}{}", KEYPADS_PATH, KEYPADS_UI_RON);
        let loaded_keypads_list = CalcKeypads::from_ron(&keypads_list_path)?;
        let mut loaded_keypads: Vec<AssembledKeypad> = Vec::new();

        //  then loop through the list and assemble each keypad
        for keypad_ref in &loaded_keypads_list.keypads {
            let assembled_keypad = AssembledKeypad::new(&keypad_ref)?;
            loaded_keypads.push(assembled_keypad);
            println!("assembled keypad {} pushed", keypad_ref.id);
        }

        Ok(AssembledKeypads {
            keypads_list: loaded_keypads_list,
            keypads: loaded_keypads,
            keypads_state: AssembledKeypadState::Loaded,
        })
    }
}

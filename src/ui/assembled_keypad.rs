// build an assembled keyboard

use cosmic::Element;
use cosmic::widget::Column;
use cosmic::widget::button;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::data_handling::GridPosition;
use crate::data_handling::KeyGrid;
use crate::data_handling::KeyGridError;
use crate::data_handling::Keypad;
use crate::data_handling::KeypadError;
use crate::data_handling::KeypadRef;
use crate::ui::assembled_keypads::KeypadAction;
use crate::ui::assembled_keypads::KeypadsMessage;

//  set up our error handling

pub type AssembledKeypadResult = Result<AssembledKeypad, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to build Keypad: {0}")]
    FailedKeypad(#[from] KeypadError),
    #[error("Failed to build KeyGrid: {0}")]
    FailedKeyGrid(#[from] KeyGridError),
}

//  bow read our Keypads structure from the RON file

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct AssembledKeypad {
    pub keypad: Keypad,
    pub keygrid: KeyGrid,
}

impl AssembledKeypad {
    // create a new assembled keypad from the RON files
    pub fn new(keypad_ref: &KeypadRef) -> Result<Self, Error> {
        let loaded_keypad = Keypad::from_ron(keypad_ref)?;
        let loaded_grid = KeyGrid::new(&loaded_keypad.keys)?;

        Ok(AssembledKeypad {
            keypad: loaded_keypad,
            keygrid: loaded_grid,
        })
    }

    // this generates the view template for the CalcKeyGrid
    // iterate through the Grid and create a button - just
    // stick it in a column for now until we get it working
    // and then we will play with rows and columns - i just
    // want to see a display of buttons
    pub fn view(&self) -> Element<KeypadsMessage> {
        let mut column_layout = Column::new();

        // iterate through the hashmap using GridPodition
        for row_iter in 1..=self.keypad.rows as usize {
            for col_iter in 1..=self.keypad.columns as usize {
                // get the CalcKey at cells(GridPosition)
                let pos = GridPosition::new(row_iter, col_iter);

                // cretae a button with the data from CalcKey and then
                // wrap it in a tooltip and push it onto the column
                if let Some(calc_key) = self.keygrid.cells.get(&pos) {
                    let button_widget: cosmic::Element<_> = button::standard(&calc_key.label)
                        .on_press(KeypadsMessage::Keypad(KeypadAction::KeyPressed(
                            pos,
                            calc_key.qalc_term.clone(),
                        )))
                        .into();
                    column_layout = column_layout.push(button_widget)
                }
            }
        }
        column_layout.into()
    }
}

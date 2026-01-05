// these are the helper functions used by AssembledKeypads
// just like the directory they are divided into Model View Update

use cosmic::Task;

use super::AssembledKeypads;
use super::KeypadAction;
use super::KeypadSet;
use super::KeypadsMessage;
use super::ListAction;

// Model Helpers

// View Helpers

// Update Helpers
// AssembledKeypads::Message::List::ListAction

impl AssembledKeypads {
    pub fn handle_list_action(
        &mut self,
        action: ListAction,
    ) -> Task<cosmic::Action<KeypadsMessage>> {
        match action {
            ListAction::Load(keypadset, keypads_path) => {
                self.create_keypads_list(keypadset, keypads_path)
            }
            ListAction::LoadFailed => Task::none(),
            ListAction::LoadedOk => Task::none(),
            ListAction::SwitchTo(keypad_id) => Task::none(),
        }
    }

    // create a new CalcKeypads and put it in self.keypads_list

    pub fn create_keypads_list(
        &mut self,
        keypadset: KeypadSet,
        path: String,
    ) -> Task<cosmic::Action<KeypadsMessage>> {
        Task::none()
    }

    // AssembledKeypads::Message::Keypad::KeypadAction

    pub fn handle_keypad_action(
        &mut self,
        action: KeypadAction,
    ) -> Task<cosmic::Action<KeypadsMessage>> {
        match action {
            KeypadAction::Assemble(keypadref) => Task::none(),
            KeypadAction::AssembledOK => Task::none(),
            KeypadAction::AssembleFailed => Task::none(),
            KeypadAction::KeyPressed(gridposition, qcalc_term) => Task::none(),
        }
    }
}

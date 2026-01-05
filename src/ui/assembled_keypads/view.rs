// buld the view for AssembledKeypads
// it will invoke the view function from AssembledKeypad
// and some helper functions to put it all together
// basically a keypad of buttons - or an error message
//
use cosmic::Element;
use cosmic::widget::Column;
use cosmic::widget::Text;

use super::AssembledKeypads;
use super::KeypadsMessage;
use super::State;

impl AssembledKeypads {
    // now the View part of the MVU for AssmbledKeypads
    // this is where we build the keypad component of the
    // UI template that the App View will use as part
    // of its template
    pub fn view(&self) -> Element<KeypadsMessage> {
        match &self.keypads_state {
            // if loading then just display a loading message
            State::Assembling => {
                let output_column = Column::new().push(Text::new("Loading Keypads..."));
                output_column.into()
            }
            // if the keypads are loaded then ask the currently active
            // keypad to rerurn it view - thanks
            State::Assembled => {
                let output_column = Column::new().push(Text::new("Loaded Keypads OK"));
                output_column.into()
            }

            // if there is an error then create some text to send up to the
            // App View so it can display the message tree
            State::Error(error) => {
                let output_column = Column::new().push(Text::new(format!("Error: {}", error)));
                output_column.into()
            }
        }
    }
}

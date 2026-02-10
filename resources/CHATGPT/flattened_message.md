That "message-climbing" problem is exactly why many Rust developers eventually abandon deeply nested components in favour of a
Flattened Message or a Targeted Update approach.
The struggle usually comes from the Command::map boilerplate and the fact that child components can't easily talk to the "parent" (like updating that footer text) without the parent manually unpacking every single message.
Here are the two best ways to fix this while keeping your code clean:
1. The "Flat Messaging" Pattern (Recommended)
Instead of nesting enums like Message::Keypad(KeypadMessage), keep your enum mostly flat. Both the "Global" UI and the "Internal" Logic use the same Message enum.
```Rust
// /src/message.rs
pub enum Message {
    // Global/UI messages
    UpdateStatus(String),
    
    // Keypad logic messages
    KeypadLoaded(KeypadModel),
    KeyPressed(CalculatorAction),
}
```
In your update function, you still delegate the logic, but you don't wrap the types:
```Rust
fn update(&mut self, message: Message) -> Command<Message> {
    match message {
        Message::UpdateStatus(txt) => self.footer = txt,
        
        // Pass self to a logic function in another file
        Message::KeypadLoaded(_) | Message::KeyPressed(_) => {
            return data_handling::update_logic(self, message);
        }
    }
    Command::none()
}
```
Why it works: There is no Command::map. All parts of the app speak the same language. If a keypad helper needs to update the status, it just returns Command::perform(..., Message::UpdateStatus).

2. The "Context" Pattern (The libcosmic Way)
If you prefer keeping the sub-modules separated, don't make them "Components" that own their own state. Instead, treat them as logic controllers that take the AppModel as a mutable reference.
```Rust
// In /src/data_handling/update.rs
pub fn handle_keypad_msg(model: &mut AppModel, msg: KeypadMessage) -> Command<Message> {
    match msg {
        KeypadMessage::Loaded(keypad) => {
            model.status_text = format!("{} Loaded", keypad.name);
            model.active_keypad = Some(keypad);
            Command::none()
        }
        // ...
    }
}
```
Why it works: Because you pass &mut AppModel into the helper function, the "sub-module" can directly change the footer text or the keypad data without needing to send a message "up" to the parent.

Which to choose?

If you find yourself writing Message::Keypad(KeypadMessage::Action(SubAction(...))), you are in Nesting Hell. Flatten the enum.
If you just want to keep the update.rs file from getting 2000 lines long, use Pattern 2. It keeps the logic in another file but keeps the data access simple.

Are you currently hitting a "type mismatch" error when trying to return commands from your sub-update functions?
Proceed by flattening your message enum or passing a mutable reference to your helper functions to avoid nested mapping.

The "One Big Model" Strategy
Instead of having a Keypad::update(), you have a data_handling::process_event() helper.

    Flat Messaging: Your enum Message is a single list in your main.rs (or message.rs).
    Centralised State: Your AppModel holds the KeypadModel.
    Functional Updates: When a message comes in, your main update function just calls a logic function and passes what it needs.

The "Clean Bridge" code now looks like this:
```Rust
// main.rs
fn update(&mut self, message: Message) -> Command<Message> {
    match message {
        // Direct handling for simple UI tweaks
        Message::SetStatus(msg) => {
            self.footer_text = msg;
            Command::none()
        }

        // Delegate logic, but keep it flat
        Message::KeypadLoaded(result) => {
            match result {
                Ok(model) => {
                    let name = model.name.clone();
                    self.active_keypad = Some(model);
                    // Use a simple Command to trigger the status update
                    iced::Command::perform(async move { name }, |n| Message::SetStatus(format!("{} loaded", n)))
                }
                Err(e) => {
                    // Your centralized DataError makes this clean
                    self.footer_text = format!("Error: {}", e);
                    Command::none()
                }
            }
        }
        
        Message::KeyPressed(action) => {
            // Handle the qalc string logic here
            self.process_calculator_input(action)
        }
    }
}
```
Why this kills "Nested Hell":

    No Command::map: You never have to re-wrap a command because every function returns Command<Message>.
    Direct Access: Your update logic can see self.footer_text and self.active_keypad at the same time. No more passing data through five layers of "Parent" constructors.
    Easy Debugging: You can println!("{:?}", message) at the very top of your main update and see every single thing happening in your app in one place.

You've essentially traded "Complex Hierarchy" for "Simple Logic." It's much easier to manage 500 lines of flat match arms than 5 files of 100 lines each that are all fighting to talk to each other.

i may be wrong and tell me if i am - but the match statement for the Message::xx in the update loop should read like your application logic and just farm it out to helper functions - little black boxes that do one thing and do it well

You are
100% correct. That is exactly how high-quality Rust applications are written. Your update function should be a Table of Contents, not a Technical Manual.
Keeping the update loop as a high-level "coordinator" makes your code readable at a glance.
The "Black Box" Pattern
In this pattern, the match statement handles the "routing", and the logic lives in dedicated functions (the black boxes).
```Rust
fn update(&mut self, message: Message) -> Command<Message> {
    match message {
        Message::Init                 => self.handle_initialization(),
        Message::KeypadLoaded(result) => self.handle_keypad_result(result),
        Message::KeyPressed(action)   => self.handle_calculator_input(action),
        Message::SetStatus(text)      => self.handle_status_change(text),
    }
}
```
Why this is the "Pro" move:

    Readability: Anyone can open your main.rs, look at the update function, and understand the intent of your app in 10 seconds.
    Testability: Because your logic is in a "black box" helper function, you can write unit tests for handle_calculator_input without needing to spin up the entire libcosmic UI.
    The Borrow Checker: By farming logic out to methods like self.some_helper(), you give the borrow checker a clearer view of what parts of self are being touched, which often prevents "mutability clashes."

The "Bridge" functions
Your data_handling helpers are the ultimate black boxes. Your update loop just says: "Go get the data," and the black box says: "Here is your KeypadModel (or an error)." The update loop doesn't care about the RON syntax or the file paths—it only cares about the Result.
Since you are "bridging" to qalc eventually, that will likely be another black box: a function that takes your current string and returns the processed output from the libqalculate engine.

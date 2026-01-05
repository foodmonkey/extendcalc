
Since you are using Iced (or cosmic, which shares the same architecture), you will follow the 
Elm Architecture. Because HashMap is unordered, you must iterate through your grid dimensions manually 
in your view function to ensure buttons appear in the correct physical sequence.

1. Define the Message

First, define a Message that will be sent to your update function when a button is pressed.

```rust
#[derive(Debug, Clone)]
pub enum Message {
    ButtonPressed(String), // The ID of the key that was pressed
}
```

2. The View Function (Assembling the Grid)

In Iced, you build the UI by nesting Row and Column widgets.
You will loop through your expected rows and columns, probing your HashMap for each coordinate.

```rust
use iced::Element;
use iced::widget::{Column, button, column, row, text};

// Inside your view function:
fn view(&self) -> Element<Message> {
    let mut grid_layout = Column::new().spacing(10); // Vertical container

    for r in 0..max_rows {
        let mut row_layout = row![].spacing(10); // Horizontal container for this row

        for c in 0..max_cols {
            // Check if there is a KeyDef at this (row, col)
            if let Some(key_def) = self.keypad.grid.get(&(r, c)) {
                // Create a button that sends a Message when clicked
                let btn = button(text(&key_def.label))
                    .on_press(Message::ButtonPressed(key_def.key_id.clone()));

                row_layout = row_layout.push(btn);
            } else {
                // Optional: Push a spacer or empty label for empty grid slots
                row_layout = row_layout.push(text(" ").width(50));
            }
        }
        grid_layout = grid_layout.push(row_layout);
    }

    grid_layout.into()
}
```

3. Handle the Event

In your update function, you can now react to the button press.

```rust
fn update(&mut self, message: Message) {
    match message {
        Message::ButtonPressed(id) => {
            println!("Key {} was activated!", id);
            // Here you would trigger the actual keypad logic (e.g., sending a command)
        }
    }
}
```

Key Implementation Notes

    Grid Consistency: 
    By using for r in 0..max_rows, you bypass the HashMap's internal random order and guarantee 
    the UI reflects your intended grid layout.

    Spacing: 
    Using .spacing(10) on Column and Row is the easiest way to get immediate visual separation 
    between your keys.

    Cosmic Specifics: 
    If using libcosmic, the syntax is nearly identical as it wraps Iced widgets, but you may have access to a specialized cosmic::widget::Grid for more complex layouts.

These guides demonstrate how to structure UI elements like buttons and rows in Iced and how to handle custom widgets and application state:

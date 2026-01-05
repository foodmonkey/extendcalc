
Next Step Tip for Iced:

Since you are using a HashMap for the grid, remember that iced's Column and Row widgets expect an iterator. 
You can now easily generate your UI like this:

```rust
// A simplified conceptual view loop
(0..total_rows).map(|r| {
    row!( (0..total_cols).map(|c| {
        if let Some(key) = assembled_keypad.grid.get(&(r, c)) {
            button(text(&key.name)).on_press(Message::KeyClicked(key.id.clone())).into()
        } else {
            horizontal_space(Length::Fill).into() // Empty cell
        }
    }))
})
```

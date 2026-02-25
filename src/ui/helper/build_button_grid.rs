use cosmic::Element;
use cosmic::iced::{Alignment, Length};
use cosmic::widget::{button, column, container, row, text, tooltip};

use crate::app::Message;
use crate::ui::GridPosition;
use crate::ui::KeyGrid;
use crate::ui::style::keypad_container_style;

pub fn build_button_grid(
    keygrid: &KeyGrid,
    _rows: usize,
    _columns: usize,
) -> Element<'static, Message> {
    let mut rows: Vec<Element<'static, Message>> = Vec::with_capacity(_rows as usize);
    rows.push(text("Keypad").into());

    for iter_row in 1..=_rows {
        let mut buttons: Vec<Element<'static, Message>> = Vec::with_capacity(_columns as usize);

        for iter_column in 1..=_columns {
            // get the Key metadata
            let grid_index = GridPosition {
                row: iter_row,
                column: iter_column,
            };
            let key = keygrid.get(&grid_index);

            let label_text = text(key.label.clone())
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center);

            let key_button = button::custom(label_text)
                .width(70.0)
                .height(40.0)
                .on_press(Message::KeyPressed(key.id.clone()));

            match key.tooltip_text.as_str() {
                "" => {
                    buttons.push(key_button.into());
                }
                _ => {
                    let key_tooltip = text(key.tooltip_text.clone());
                    let key_with_tooltip = tooltip(key_button, key_tooltip, tooltip::Position::Top)
                        .gap(10)
                        .into();

                    buttons.push(key_with_tooltip);
                }
            }
        }

        rows.push(
            row::with_children(buttons)
                .spacing(10)
                .width(Length::Shrink)
                .into(),
        );
    }
    let keypad = column::with_children(rows)
        .spacing(10)
        .width(Length::Shrink)
        .height(Length::Shrink);

    container(keypad)
        .padding(15)
        .style(keypad_container_style)
        .into()
}

// so this function gets passed as a callback to the .style on a container
// so the current theme gets passed in by cosmic - nice

use cosmic::iced::{Background, Border, Color, Radius};
use cosmic::theme::Theme;
use cosmic::widget::container;

pub fn keypad_container_style(theme: &Theme) -> container::Style {
    let cosmic = theme.cosmic();

    let background_color: Color = cosmic.secondary_container_color().into();
    // not sure if i want borders around the keypads
    // let border_color: Color = cosmic.secondary_container_divider().into();
    let border_color: Color = Color::TRANSPARENT;
    let radius: Radius = cosmic.radius_s().into();

    container::Style {
        background: Some(Background::Color(background_color)),
        border: Border {
            color: border_color,
            width: 0.0,
            radius: radius,
        },
        ..container::Style::default()
    }
}

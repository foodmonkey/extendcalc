// the style sheet for the Key-Button widget
use cosmic::Theme;
use cosmic::iced::{Background, Color, Radius, Vector};
use cosmic::widget::button;

pub fn key_button_style(theme: &Theme) -> button::Style {
    let cosmic = theme.cosmic();

    let bg_color: Color = cosmic.bg_color().into();
    let border_color: Color = cosmic.bg_divider().into();
    let text_color: Color = cosmic.bg_component_color().into();
    let radius: Radius = cosmic.radius_s().into();

    button::Style {
        shadow_offset: Vector::new(0.0, 1.0),
        background: Some(Background::Color(bg_color)),
        overlay: None,
        border_radius: radius,
        border_width: 1.0,
        border_color: border_color,
        outline_width: 0.0,
        outline_color: Color::TRANSPARENT,
        icon_color: Some(Color::TRANSPARENT),
        text_color: Some(text_color),
    }
}

use cosmic::iced::Border;
use cosmic::theme::Theme;
use cosmic::widget::container;

pub fn keypad_container_style(theme: &Theme) -> container::Style {
    container::Style {
        border: Border {
            // Using the theme's variant for a clean look
            color: theme.cosmic().primary_component_color().into(),
            width: 2.0,
            radius: 8.0.into(),
        },
        ..container::Style::default()
    }
}

use iced::{
    widget::button::{Status, Style},
    Theme, {Background, Border, Color},
};

pub fn secondary(theme: &Theme, status: Status, selected: bool) -> Style {
    let palette = theme.extended_palette();

    let foreground = palette.secondary.base.text;
    let button_colors = palette.secondary;

    let background = if selected {
        button_colors.strong.color
    } else {
        button_colors.weak.color
    };

    let background_hover = button_colors.base.color;

    button(foreground, background, background_hover, status)
}

fn button(foreground: Color, background: Color, background_hover: Color, status: Status) -> Style {
    match status {
        Status::Active | Status::Pressed => Style {
            background: Some(Background::Color(background)),
            text_color: foreground,
            border: Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        },
        Status::Hovered => Style {
            background: Some(Background::Color(background_hover)),
            text_color: foreground,
            border: Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        },
        Status::Disabled => {
            let active: Style = button(foreground, background, background_hover, Status::Active);

            Style {
                text_color: Color {
                    a: 0.2,
                    ..active.text_color
                },
                ..active
            }
        }
    }
}

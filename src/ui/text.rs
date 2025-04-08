use iced::{widget::text::Style, Theme};

pub fn none(_theme: &Theme) -> Style {
    Style { color: None }
}

pub fn primary(theme: &Theme) -> Style {
    Style {
        color: Some(theme.extended_palette().primary.base.color),
    }
}

pub fn secondary(theme: &Theme) -> Style {
    Style {
        color: Some(theme.extended_palette().secondary.base.color),
    }
}

pub fn tertiary(theme: &Theme) -> Style {
    Style {
        color: Some(theme.extended_palette().warning.base.color),
    }
}

pub fn error(theme: &Theme) -> Style {
    Style {
        color: Some(theme.extended_palette().danger.base.color),
    }
}

pub fn success(theme: &Theme) -> Style {
    Style {
        color: Some(theme.extended_palette().success.base.color),
    }
}

pub fn action(theme: &Theme) -> Style {
    Style {
        color: Some(theme.extended_palette().primary.strong.color),
    }
}

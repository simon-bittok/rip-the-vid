use iced::{
    Border, Color, Shadow, Theme, Vector,
    widget::{
        button::{self, Status, Style},
        container,
    },
};

pub fn button_style(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.extended_palette();
    button::Style {
        background: Some(match status {
            button::Status::Hovered => palette.primary.strong.color.into(),
            button::Status::Pressed => palette.primary.base.color.into(),
            _ => palette.primary.weak.color.into(),
        }),
        text_color: palette.primary.strong.text,
        border: Border::default().rounded(6),
        shadow: Shadow::default(),
    }
}

pub fn exit_button_style(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.extended_palette();
    button::Style {
        background: Some(match status {
            button::Status::Hovered => palette.danger.strong.color.into(),
            button::Status::Pressed => palette.danger.base.color.into(),
            _ => palette.danger.weak.color.into(),
        }),
        text_color: palette.danger.strong.text,
        border: Border::default().rounded(6),
        shadow: Shadow::default(),
    }
}

pub fn jrip_button_style(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.extended_palette();
    button::Style {
        background: Some(match status {
            button::Status::Hovered => palette.success.weak.color.into(),
            button::Status::Pressed => palette.success.base.color.into(),
            _ => palette.success.base.color.into(),
        }),
        text_color: palette.success.strong.text,
        border: Border::default().rounded(6),
        shadow: Shadow::default(),
    }
}

pub fn close_button_style(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.extended_palette();
    button::Style {
        background: Some(match status {
            button::Status::Hovered => palette.success.weak.color.into(),
            _ => palette.success.base.color.into(),
        }),
        text_color: palette.success.strong.text,
        border: Border::default().rounded(4),
        shadow: Shadow::default(),
    }
}

pub fn header_style(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();
    container::Style {
        background: Some(palette.background.strong.color.into()),
        border: Border::default(),
        ..Default::default()
    }
}

pub fn popup_style(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();
    container::Style {
        background: Some(palette.success.weak.color.into()),
        border: Border {
            color: palette.success.strong.color,
            width: 1.0,
            radius: 6.0.into(),
        },
        ..Default::default()
    }
}

pub fn file_row_style() -> impl Fn(&Theme, button::Status) -> Style {
    |theme, status| {
        let palette = theme.extended_palette();
        button::Style {
            background: Some(match status {
                button::Status::Hovered => Color::from_rgba(0.3, 0.5, 0.8, 0.15).into(),
                _ => Color::TRANSPARENT.into(),
            }),
            text_color: palette.background.base.text,
            border: match status {
                button::Status::Hovered => Border::default()
                    .color(Color::from_rgba(0.3, 0.5, 0.8, 0.2))
                    .width(1)
                    .rounded(12),
                _ => Border::default(),
            },
            shadow: Shadow::default(),
        }
    }
}

pub fn dir_button_style() -> impl Fn(&Theme, Status) -> Style {
    |theme, status| {
        let palette = theme.extended_palette();
        button::Style {
            background: Some(match status {
                button::Status::Hovered => Color::from_rgba(0.2, 0.5, 0.8, 0.2).into(),
                button::Status::Pressed => Color::from_rgba(0.2, 0.5, 0.8, 0.3).into(),
                _ => Color::TRANSPARENT.into(),
            }),
            text_color: palette.primary.strong.color,
            border: match status {
                button::Status::Hovered | button::Status::Pressed => Border::default()
                    .color(Color::from_rgba(0.3, 0.5, 0.8, 0.2))
                    .width(1)
                    .rounded(12),
                _ => Border::default(),
            },
            shadow: Shadow::default(),
        }
    }
}

pub fn sidebar_style(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();
    container::Style {
        background: Some(palette.background.weak.color.into()),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 0.0.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
            offset: Vector::new(2.0, 0.0),
            blur_radius: 8.0,
        },
        ..Default::default()
    }
}

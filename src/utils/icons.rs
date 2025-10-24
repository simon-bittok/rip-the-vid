use iced::{Element, Font, widget::text};

use crate::message::Message;

fn icon<'a>(code_point: char) -> Element<'a, Message> {
    const ICON_FONT: Font = Font::with_name("jrip-icons");

    text(code_point).font(ICON_FONT).into()
}

pub fn folder_icon<'a>() -> Element<'a, Message> {
    icon('\u{E800}')
}

pub fn music_icon<'a>() -> Element<'a, Message> {
    icon('\u{E801}')
}

pub fn tick_icon<'a>() -> Element<'a, Message> {
    icon('\u{E802}')
}

pub fn back_arrow_icon<'a>() -> Element<'a, Message> {
    icon('\u{E803}')
}

pub fn video_icon<'a>() -> Element<'a, Message> {
    icon('\u{E804}')
}

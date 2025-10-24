use jrip::AppState;

fn main() -> iced::Result {
    iced::application("jrip", AppState::update, AppState::view)
        .theme(AppState::theme)
        .font(include_bytes!("../../font/jrip-icons.ttf"))
        .run()
}

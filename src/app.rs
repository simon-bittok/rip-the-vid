use std::path::Path;

use iced::{
    Alignment, Element,
    Length::Fill,
    Task, Theme,
    widget::{button, column, container, horizontal_rule, row, scrollable, text, vertical_rule},
    window,
};

use crate::{
    Message,
    states::{MainState, SidebarState},
    utils,
};

#[derive(Debug)]
pub struct AppState {
    main_state: MainState,
    sidebar_state: SidebarState,
    popup: Option<String>,
}

impl Default for AppState {
    fn default() -> Self {
        let main_state = MainState::new();
        let sidebar_state = SidebarState::default();

        Self {
            main_state,
            popup: None,
            sidebar_state,
        }
    }
}

impl AppState {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Exit => window::get_latest().and_then(window::close),
            Message::CD(path_buf) => {
                self.main_state.set_current_dir(path_buf);
                self.main_state
                    .set_current_files(utils::get_files(self.main_state.current_dir()));
                Task::none()
            }
            Message::JRIP(path_buf) => {
                let file_path = Path::new(path_buf.file_name().unwrap());
                // File name without extension
                let file_name = file_path.file_stem().unwrap().to_str().unwrap();

                if let Some(parent) = path_buf.parent() {
                    let mut new_file = parent.to_path_buf();
                    new_file.push(format!("{file_name}.mp3"));

                    if let Ok(output) = utils::rip_audio(path_buf, new_file) {
                        if output.success() {
                            self.popup = Some(String::from("Audio ripped successfully"))
                        } else {
                            self.popup = Some(String::from("Audio ripping failed"))
                        }
                    }
                }

                Task::none()
            }
            Message::ClosePopup => {
                self.popup = None;
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let header = container(
            row![
                text(
                    self.main_state
                        .current_dir()
                        .to_str()
                        .unwrap_or("unkown directory")
                )
                .size(24)
                .width(Fill),
                // back button
                button(utils::back_arrow_icon())
                    .padding([8, 16])
                    .style(utils::button_style)
                    .on_press(Message::CD(
                        self.main_state
                            .current_dir()
                            .parent()
                            .unwrap_or(self.main_state.current_dir())
                            .to_path_buf()
                    )),
                // exit button - redundant
                button(text("Exit").size(16))
                    .padding([8, 16])
                    .style(utils::exit_button_style)
                    .on_press(Message::Exit)
            ]
            .spacing(12)
            .align_y(Alignment::Center),
        )
        .padding(16)
        .style(utils::header_style);

        let file_list = self.main_state.view();

        let mut main_content = column![header, horizontal_rule(1)].spacing(0);

        if let Some(popup) = &self.popup {
            let popup_widget = container(
                row![
                    text(popup).size(16).width(Fill),
                    button(utils::tick_icon())
                        .padding([6, 12])
                        .style(utils::close_button_style)
                        .on_press(Message::ClosePopup)
                ]
                .spacing(12)
                .align_y(Alignment::Center),
            )
            .padding(16)
            .style(utils::popup_style);

            main_content = main_content.push(popup_widget);
        }

        main_content = main_content
            .push(scrollable(container(file_list).padding(16).width(Fill)).height(Fill));

        let sidebar = self.sidebar_state.view();

        row![
            sidebar,
            vertical_rule(1),
            container(main_content).width(Fill).height(Fill),
        ]
        .spacing(0)
        .into()
    }

    pub fn theme(&self) -> Theme {
        Theme::CatppuccinMocha
    }
}

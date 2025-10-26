use std::path::PathBuf;

use iced::{
    Alignment, Color, Element,
    Length::Fill,
    widget::{button, column, container, row, text},
};

use crate::{Message, utils};

#[derive(Debug, Clone)]
pub struct MainState {
    current_dir: PathBuf,
    current_files: Vec<(String, bool)>,
}

impl MainState {
    pub fn new() -> Self {
        let current_dir = std::env::current_dir().unwrap();
        let current_files = utils::get_files(&current_dir);

        Self {
            current_dir,
            current_files,
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let mut file_list = column![].spacing(4);

        for (file, is_dir) in self.current_files() {
            let file_name = text(file).size(16);
            let mut file_path = self.current_dir().to_owned();
            file_path.push(file);

            if *is_dir {
                let dir_row = container(
                    button(
                        row![
                            button(utils::folder_icon()).style(|_, _| button::Style {
                                background: None,
                                text_color: Color::from_rgb(0.4, 0.6, 1.0),
                                ..Default::default()
                            }),
                            file_name
                        ]
                        .spacing(8)
                        .align_y(Alignment::Center),
                    )
                    .width(Fill)
                    .padding([4, 8])
                    .style(utils::dir_button_style())
                    .on_press(Message::CD(file_path)),
                )
                .padding([4, 8]);

                file_list = file_list.push(dir_row);
            } else {
                let file_row = container(
                    row![
                        button(utils::video_icon()).style(|_theme, _| button::Style {
                            background: None,
                            text_color: Color::from_rgb(0.4, 0.6, 1.0), // Blue accent
                            ..Default::default()
                        }),
                        file_name.width(Fill),
                        button(row![utils::music_icon(), text("Extract Audio")].spacing(8))
                            .padding([6, 12])
                            .style(utils::jrip_button_style)
                            .on_press(Message::JRIP(file_path))
                    ]
                    .spacing(16)
                    .align_y(Alignment::Center)
                    .padding([4, 8]),
                )
                .padding([4, 8])
                .width(Fill);

                file_list = file_list.push(file_row);
            }
        }

        file_list.into()
    }

    pub fn current_dir(&self) -> &PathBuf {
        &self.current_dir
    }

    pub fn current_files(&self) -> &Vec<(String, bool)> {
        &self.current_files
    }

    pub fn set_current_dir(&mut self, current_dir: PathBuf) {
        self.current_dir = current_dir;
    }

    pub fn set_current_files(&mut self, current_files: Vec<(String, bool)>) {
        self.current_files = current_files;
    }
}

impl Default for MainState {
    fn default() -> Self {
        Self::new()
    }
}

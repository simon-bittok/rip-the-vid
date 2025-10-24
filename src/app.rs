use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use iced::{
    Alignment, Color, Element,
    Length::Fill,
    Task, Theme,
    widget::{button, column, container, horizontal_rule, row, scrollable, text},
    window,
};

use crate::{Message, utils};

#[derive(Debug)]
pub struct AppState {
    current_dir: PathBuf,
    current_files: Vec<(String, bool)>,
    popup: Option<String>,
    sidebar_dir: Option<PathBuf>,
    sidebar_files: Vec<(String, bool)>,
}

impl Default for AppState {
    fn default() -> Self {
        let current_dir = std::env::current_dir().unwrap();
        let current_files = get_files(&current_dir);

        Self {
            current_dir,
            current_files,
            popup: None,
            sidebar_dir: None,
            sidebar_files: Vec::new(),
        }
    }
}

impl AppState {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Exit => window::get_latest().and_then(window::close),
            Message::CD(path_buf) => {
                self.current_dir = path_buf;
                self.current_files = get_files(&self.current_dir);
                Task::none()
            }
            Message::JRIP(path_buf) => {
                let file_path = Path::new(path_buf.file_name().unwrap());
                // File name without extension
                let file_name = file_path.file_stem().unwrap().to_str().unwrap();

                if let Some(parent) = path_buf.parent() {
                    let mut new_file = parent.to_path_buf();
                    new_file.push(format!("{file_name}.mp3"));

                    if let Ok(output) = Command::new("ffmpeg")
                        .args([
                            "-i",
                            path_buf.to_str().unwrap_or("/home"),
                            "-y",
                            new_file.to_str().unwrap_or("/home"),
                        ])
                        .status()
                    {
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
            Message::ShowSidebar(path_buf) => {
                self.sidebar_files = get_files(&path_buf);
                self.sidebar_dir = Some(path_buf);
                Task::none()
            }
            Message::CloseSidebar => {
                self.sidebar_dir = None;
                self.sidebar_files.clear();
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let header = container(
            row![
                text(self.current_dir.to_str().unwrap_or("unkown directory"))
                    .size(24)
                    .width(Fill),
                // back button
                button(utils::back_arrow_icon())
                    .padding([8, 16])
                    .style(utils::button_style)
                    .on_press(Message::CD(
                        self.current_dir
                            .parent()
                            .unwrap_or(&self.current_dir)
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

        let mut file_list = column![].spacing(8);

        for (file, is_dir) in &self.current_files {
            let file_name = text(file).size(16);
            let mut file_path = self.current_dir.clone();
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
                            file_name.width(Fill),
                            button("Open")
                                .padding([6, 12])
                                .style(utils::button_style)
                                .on_press(Message::ShowSidebar(file_path.clone()))
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
                    button(
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
                    .width(Fill)
                    .style(utils::file_row_style()),
                )
                .padding([4, 8]);

                file_list = file_list.push(file_row);
            }
        }

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

        if let Some(sidebar_dir) = &self.sidebar_dir {
            let sidebar_header = container(
                row![
                    text(
                        sidebar_dir
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("Folder")
                    )
                    .size(20)
                    .width(Fill),
                    button(text("X").size(16))
                        .padding([6, 12])
                        .style(utils::close_button_style)
                        .on_press(Message::CloseSidebar)
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            )
            .padding(12)
            .style(utils::header_style);

            let mut sidebar_list = column![].spacing(4);

            for (file, is_dir) in &self.sidebar_files {
                let file_name = text(file).size(14);
                let mut file_path = sidebar_dir.clone();
                file_path.push(file);

                if *is_dir {
                    let item = container(
                        button(
                            row![button(utils::folder_icon()), file_name]
                                .spacing(6)
                                .align_y(Alignment::Center),
                        )
                        .width(Fill)
                        .padding([8, 10])
                        .style(utils::dir_button_style())
                        .on_press(Message::CD(file_path)),
                    )
                    .padding([1, 0]);

                    sidebar_list = sidebar_list.push(item);
                } else {
                    let item = container(
                        row![button(utils::video_icon()), file_name]
                            .spacing(6)
                            .align_y(Alignment::Center)
                            .padding([8, 10]),
                    )
                    .padding([1, 0]);

                    sidebar_list = sidebar_list.push(item);
                }
            }

            let sidebar = container(
                column![
                    sidebar_header,
                    horizontal_rule(1),
                    scrollable(container(sidebar_list).padding(12).width(Fill)).height(Fill)
                ]
                .spacing(10),
            )
            .width(300)
            .height(Fill)
            .style(utils::sidebar_style);

            row![container(main_content).width(Fill).height(Fill), sidebar]
                .spacing(0)
                .into()
        } else {
            container(main_content).width(Fill).height(Fill).into()
        }
    }

    pub fn theme(&self) -> Theme {
        Theme::CatppuccinMacchiato
    }
}

fn get_files(path: &PathBuf) -> Vec<(String, bool)> {
    let mut dirs = Vec::default();
    let mut files = Vec::default();

    if let Ok(read_dir) = fs::read_dir(path) {
        for dir_entry in read_dir.flatten() {
            if let Some(name) = dir_entry.file_name().to_str() {
                if dir_entry.path().is_dir() {
                    dirs.push((name.to_string(), true));
                } else if name.ends_with(".mkv") || name.ends_with(".MKV") {
                    dirs.push((name.to_string(), false));
                }
            }
        }
    }

    dirs.append(&mut files);
    dirs
}

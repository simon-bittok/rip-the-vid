use std::path::PathBuf;

use iced::{
    Alignment, Element,
    Length::Fill,
    widget::{button, column, container, horizontal_rule, row, scrollable, text},
};

use crate::{Message, utils};

#[derive(Debug)]
pub struct SidebarState {
    sidebar_dir: PathBuf,
    sidebar_files: Vec<(String, bool)>,
}

impl SidebarState {
    pub fn new() -> Self {
        let sidebar_dir = std::env::home_dir().unwrap();
        let sidebar_files = utils::get_side_bar_dirs_and_files(&sidebar_dir);

        SidebarState {
            sidebar_dir,
            sidebar_files,
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let mut sidebar_list = column![].spacing(4);

        for (file, is_dir) in &self.sidebar_files {
            let file_name = text(file).size(14);
            let mut file_path = self.sidebar_dir.clone();
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

        container(
            column![
                horizontal_rule(1),
                scrollable(container(sidebar_list).padding(12).width(Fill)).height(Fill)
            ]
            .spacing(10),
        )
        .width(280)
        .height(Fill)
        .style(utils::sidebar_style)
        .into()
    }

    pub fn sidebar_dir(&self) -> &PathBuf {
        &self.sidebar_dir
    }

    pub fn sidebar_files(&self) -> &Vec<(String, bool)> {
        &self.sidebar_files
    }

    pub fn set_sidebar_dir<P: Into<PathBuf>>(&mut self, path: P) {
        self.sidebar_dir = path.into()
    }

    pub fn set_sidebar_files(&mut self, files: Vec<(String, bool)>) {
        self.sidebar_files = files;
    }
}

impl Default for SidebarState {
    fn default() -> Self {
        Self::new()
    }
}

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Message {
    Exit,
    CD(PathBuf),
    JRIP(PathBuf),
    ClosePopup,
    ShowSidebar(PathBuf),
    CloseSidebar,
}

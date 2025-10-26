use std::{
    io,
    path::PathBuf,
    process::{Command, ExitStatus},
};

pub fn rip_audio(video_file: PathBuf, new_file_name: PathBuf) -> Result<ExitStatus, io::Error> {
    Command::new("ffmpeg")
        .args([
            "-i",
            video_file.to_str().unwrap_or("/home"),
            "-y",
            new_file_name.to_str().unwrap_or("/home"),
        ])
        .status()
}

use std::process::Command;

fn main() {
    let video_file = &"/Users/danielsuchan/Projects/Syncoli/syncoli-rust/subway.mp4";
    Command::new("vlc")
        .arg(video_file)
        .spawn()
        .expect("failed to execute process");
}

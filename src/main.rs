use std::process::Command;

fn main() {
    let video_file = &"/home/master/subway.mp4";
    Command::new("cvlc")
        .arg(video_file)
        .spawn()
        .expect("failed to execute process");
}

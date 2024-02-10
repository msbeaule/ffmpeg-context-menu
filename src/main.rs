use std::process::Command;
use clap::Parser;
use std::io::{stdin, stdout, Read, Write};
use std::ffi::OsStr;

/// Pauses the terminal so the user can read the output before the terminal closes
fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to exit...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

#[derive(Parser)]
struct Cli {
    /// The path to the video file to convert
    path: std::path::PathBuf,
    /// The ffmpeg conversion to run
    pattern: String,
}

/// Returns path, file name, and extension in a tuple
fn get_full_path_parts(path: &std::path::PathBuf) -> (&OsStr, &OsStr, &OsStr) {
    let path = path;

    let full_path = path.parent().unwrap().as_os_str();
    let file_name = path.file_stem().unwrap();
    let extension = path.extension().unwrap();
    
    // println!("{}", filename.to_str().unwrap());

    return (full_path, file_name, extension);
}

fn main() {
    let args = Cli::parse();

    // println!("path: {:?}, pattern: {:?}", args.path, args.pattern);

    // Runs the ffmpeg convert based on what is supplied in the args
    match args.pattern.as_str() {
        "new" => convert_to_new(&args.path),
        "half_size" => convert_to_new_half_size(&args.path),
        "audio" => convert_to_audio_only(&args.path),
        "mp4" => convert_to_mp4(&args.path),
        _=> println!("wrong option entered, refer to README"),
    };

    pause();
}

/// Makes a new video file with -crf 28 (compressed but still a good quality)
fn convert_to_new(original_full_path: &std::path::PathBuf) {
    let (path, file_name, extension) = get_full_path_parts(original_full_path);

    // println!("path: {:?}, file_name: {:?}, extension: {:?}", path, file_name, extension);

    let new_name = format!("{}/{}-CONVERTED.{}", path.to_str().unwrap(), file_name.to_str().unwrap(), extension.to_str().unwrap());

    let mut child = Command::new("ffmpeg")
        .arg("-i")
        .arg(original_full_path)
        .arg("-map")
        .arg("0")
        .arg("-map_metadata")
        .arg("0")
        .arg("-crf")
        .arg("28")
        .arg(new_name)
        .spawn()
        .unwrap();

    let _result = child.wait().unwrap();
}

/// Makes a new video file with -crf 28 with half the original width
fn convert_to_new_half_size(original_full_path: &std::path::PathBuf) {
    let (path, file_name, extension) = get_full_path_parts(original_full_path);

    let new_name = format!("{}/{}-HALF-SIZE.{}", path.to_str().unwrap(), file_name.to_str().unwrap(), extension.to_str().unwrap());

    let mut child = Command::new("ffmpeg")
        .arg("-i")
        .arg(original_full_path)
        .arg("-vf")
        .arg("scale=iw/2:-2")
        .arg("-map")
        .arg("0")
        .arg("-map_metadata")
        .arg("0")
        .arg("-crf")
        .arg("28")
        .arg(new_name)
        .spawn()
        .unwrap();

    let _result = child.wait().unwrap();
}

fn convert_to_audio_only(original_full_path: &std::path::PathBuf) {
    let (path, file_name, _) = get_full_path_parts(original_full_path);

    let new_name = format!("{}/{}-audio.mp3", path.to_str().unwrap(), file_name.to_str().unwrap());

    let mut child = Command::new("ffmpeg")
        .arg("-i")
        .arg(original_full_path)
        .arg("-vn")
        .arg("-acodec")
        .arg("libmp3lame")
        .arg(new_name)
        .spawn()
        .unwrap();

    let _result = child.wait().unwrap();
}

fn convert_to_mp4(original_full_path: &std::path::PathBuf) {
    let (path, file_name, _) = get_full_path_parts(original_full_path);

    let new_name = format!("{}/{}.mp4", path.to_str().unwrap(), file_name.to_str().unwrap());

    let mut child = Command::new("ffmpeg")
        .arg("-i")
        .arg(original_full_path)
        .arg("-f")
        .arg("mp4")
        .arg(new_name)
        .spawn()
        .unwrap();

    let _result = child.wait().unwrap();
}

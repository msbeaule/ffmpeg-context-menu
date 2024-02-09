use std::process::Command;
use clap::Parser;
use std::io::{stdin, stdout, Read, Write};
use std::ffi::OsStr;

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to exit...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

#[derive(Parser)]
struct Cli {
    /// The path to the file to read
    path: std::path::PathBuf,
    /// The pattern to look for
    pattern: String,
}

/// Returns path, file name, and extension in a tuple
fn get_full_path_parts(path: &std::path::PathBuf) -> (&OsStr, &OsStr, &OsStr) {
    let path = path;
    let filename = path.file_name().unwrap();

    let full_path = path.parent().unwrap().as_os_str();

    let file_name = path.file_stem().unwrap();
    let extension = path.extension().unwrap();
    
    // println!("{}", filename.to_str().unwrap());

    return (full_path, file_name, extension);
}

fn main() {
    let args = Cli::parse();

    // println!("path: {:?}, pattern: {:?}", args.path, args.pattern);

    match args.pattern.as_str() {
        "new" => convert_to_new(&args.path),
        "half_size" => convert_to_new_half_size(&args.path),
        _=> println!("wrong option entered, refer to README"),
    };

    pause();
}

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

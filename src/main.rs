use std::process::Command;
use clap::Parser;
use std::io::{stdin, stdout, Read, Write};
use std::ffi::OsStr;
use std::process::Stdio;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::exit;

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
        "no_audio" => convert_to_no_audio(&args.path),
        "fix" => fix_video(&args.path),
        "265" => convert_to_265(&args.path),
        "remove_border" => find_and_remove_black_borders(&args.path),
        "remove_border_faster" => find_and_remove_black_borders_faster(&args.path),
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
        //.arg("-map")
        //.arg("0")
        //.arg("-map_metadata")
        //.arg("0")
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

/// Converts to audio only as a .mp3
fn convert_to_audio_only(original_full_path: &std::path::PathBuf) {
    let (path, file_name, _) = get_full_path_parts(original_full_path);

    let new_name = format!("{}/{}.mp3", path.to_str().unwrap(), file_name.to_str().unwrap());

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

/// Makes a new video file in the .mp4 format
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

/// Makes a new video file with the copied video but no audio
fn convert_to_no_audio(original_full_path: &std::path::PathBuf) {
    let (path, file_name, extension) = get_full_path_parts(original_full_path);

    let new_name = format!("{}/{}-NO-AUDIO.{}", path.to_str().unwrap(), file_name.to_str().unwrap(), extension.to_str().unwrap());

    let mut child = Command::new("ffmpeg")
        .arg("-i")
        .arg(original_full_path)
        .arg("-map")
        .arg("0:v")
        .arg("-map_metadata")
        .arg("0")
        .arg("-crf")
        .arg("28")
        .arg(new_name)
        .spawn()
        .unwrap();

    let _result = child.wait().unwrap();
}

/// Copies the original video, repairing it if needed by ignoring errors
fn fix_video(original_full_path: &std::path::PathBuf) {
    let (path, file_name, extension) = get_full_path_parts(original_full_path);

    let new_name = format!("{}/{}-FIXED.{}", path.to_str().unwrap(), file_name.to_str().unwrap(), extension.to_str().unwrap());

    let mut child = Command::new("ffmpeg")
        .arg("-err_detect")
        .arg("ignore_err")
        .arg("-i")
        .arg(original_full_path)
        .arg("-c")
        .arg("copy")
        .arg(new_name)
        .spawn()
        .unwrap();

    let _result = child.wait().unwrap();
}

/// Converts video to 265 format
fn convert_to_265(original_full_path: &std::path::PathBuf) {
    let (path, file_name, extension) = get_full_path_parts(original_full_path);

    let new_name = format!("{}/{}-h.265.{}", path.to_str().unwrap(), file_name.to_str().unwrap(), extension.to_str().unwrap());

    let mut child = Command::new("ffmpeg")
        .arg("-i")
        .arg(original_full_path)
        .arg("-c:v")
        .arg("libx265")
        .arg("-map_metadata")
        .arg("0")
        .arg("-vtag")
        .arg("hvc1")
        .arg("-c:a")
        .arg("copy")
        .arg("-vcodec")
        .arg("hevc_nvenc")
        .arg(new_name)
        .spawn()
        .unwrap();

    let _result = child.wait().unwrap();
}

/// Finds where to crop out the black borders in the video by creating a dummy file, then crops the video
fn find_and_remove_black_borders(original_full_path: &std::path::PathBuf) {
    let (path, file_name, extension) = get_full_path_parts(original_full_path);

    let new_name = format!("{}/{}-CROPPED.{}", path.to_str().unwrap(), file_name.to_str().unwrap(), extension.to_str().unwrap());

    let success = _get_crop(original_full_path, extension);

    let crop_value = success.unwrap();
    if crop_value.is_empty() {
        println!("No black borders in this video found, exiting...");
        exit(0);
    }
    let crop: Vec<&str> = crop_value.split_whitespace().collect();

    let mut child = Command::new("ffmpeg")
        .arg("-i")
        .arg(original_full_path)
        .arg("-vf")
        .arg(crop[13]) // get the 13th value, this is the crop value
        .arg(new_name)
        .spawn()
        .unwrap();

    let _result = child.wait().unwrap();

}

/// internal function for the remove black borders function
fn _get_crop(original_full_path: &std::path::PathBuf, extension: &OsStr) -> Result<String, Error> {
    // get the crop info first
    let child = Command::new("ffmpeg")
        .arg("-i")
        .arg(original_full_path)
        .arg("-vf")
        .arg("cropdetect=18:2:0")
        .arg("-y") // -y to auto replace existing file
        .arg(format!("dummy.{}", extension.to_str().unwrap()))
        .stderr(Stdio::piped())
        .spawn()?
        .stderr
        .ok_or_else( || Error::new(ErrorKind::Other,"Could not capture ffmpeg output for crop value."))?;

    let reader = BufReader::new(child);

    let needle = "Parsed_cropdetect_0";

    let mut last_crop_line = "".to_string();

    for line in reader.lines() {
        let unwrapped = line.unwrap();
        if unwrapped.contains(needle) {
            last_crop_line = unwrapped;
        }
    }

    return Ok(last_crop_line);
}


/// Finds where to crop out the black borders in the video by creating a dummy file, then crops the video
fn find_and_remove_black_borders_faster(original_full_path: &std::path::PathBuf) {
    let (path, file_name, extension) = get_full_path_parts(original_full_path);

    let new_name = format!("{}/{}-CROPPED.{}", path.to_str().unwrap(), file_name.to_str().unwrap(), extension.to_str().unwrap());

    let success = _get_crop_faster(original_full_path, extension);

    let crop_value = success.unwrap();
    if crop_value.is_empty() {
        println!("No black borders in this video found, exiting...");
        exit(0);
    }
    let crop: Vec<&str> = crop_value.split_whitespace().collect();

    let mut child = Command::new("ffmpeg")
        .arg("-i")
        .arg(original_full_path)
        .arg("-vf")
        .arg(crop[13]) // get the 13th value, this is the crop value
        .arg(new_name)
        .spawn()
        .unwrap();

    let _result = child.wait().unwrap();

}

/// internal function for the remove black borders faster function
fn _get_crop_faster(original_full_path: &std::path::PathBuf, extension: &OsStr) -> Result<String, Error> {
    // get the crop info first

    // ffmpeg -ss 90 -i input.mp4 -vframes 10 -vf cropdetect
    let child = Command::new("ffmpeg")
        .arg("-ss")
        .arg("90")
        .arg("-i")
        .arg(original_full_path)
        .arg("-vframes")
        .arg("10")
        .arg("-vf")
        .arg("cropdetect=18:2:0")
        .arg("-y") // -y to auto replace existing file
        .arg(format!("dummy.{}", extension.to_str().unwrap()))
        .stderr(Stdio::piped())
        .spawn()?
        .stderr
        .ok_or_else( || Error::new(ErrorKind::Other,"Could not capture ffmpeg output for crop value."))?;

    let reader = BufReader::new(child);

    let needle = "Parsed_cropdetect_0";

    let mut last_crop_line = "".to_string();

    for line in reader.lines() {
        let unwrapped = line.unwrap();
        if unwrapped.contains(needle) {
            last_crop_line = unwrapped;
        }
    }

    return Ok(last_crop_line);
}

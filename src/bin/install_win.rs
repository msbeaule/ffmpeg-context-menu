use std::io::{stdin, stdout, Read, Write};
use std::{io, vec};
use std::path::Path;
use std::env;
use winreg::enums::*;
use winreg::RegKey;

/// Pauses the terminal so the user can read the output before the terminal closes
fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to exit...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn main() -> io::Result<()> {
    let path_to_binary_directory = env::current_dir()?;

    let the_values = vec!(
        vec!("convertToNew", "Convert to new", "\"new\""),
        vec!("convertToHalfSize", "Convert to half size", "\"half_size\""),
        vec!("convertToAudioOnly", "Convert to audio only", "\"audio\""),
        vec!("convertToMP4", "Convert to MP4", "\"mp4\""),
        vec!("convertToNoAudio", "Convert to no audio", "\"no_audio\""),
        vec!("convertFix", "Fix certain video issues", "\"fix\""),
        vec!("convertTo265", "Convert to 265", "\"265\""),
        vec!("convertRemoveBorder", "Remove the black borders around video", "\"remove_border\""),
    );

    // need to add more video extensions if needed
    let the_file_extensions = vec!(".mov", ".mp4", ".mkv");

    let hkcu = RegKey::predef(HKEY_CLASSES_ROOT);
    let base_path = Path::new("SystemFileAssociations");

    // loop through each extension to add this as a right click option
    for extension in the_file_extensions {
        let path: &Path = &base_path.join(extension).join("shell").join("ffmpegContextMenuRust");

        let (key, disp) = hkcu.create_subkey(&path)?;

        match disp {
            REG_CREATED_NEW_KEY => println!("A new key for {} has been created", extension),
            REG_OPENED_EXISTING_KEY => println!("An existing key for {} has been opened and edited", extension),
        }
        
        key.set_value("MUIVerb", &"Convert with ffmpeg (rust)")?;
        key.set_value("SubCommands", &"")?;

        let mut path_to_binary:String = "".to_owned();
        path_to_binary.push_str(path_to_binary_directory.to_str().unwrap());
        path_to_binary.push_str("\\ffmpeg-context-menu.exe \"%1\" ");

        let (shell_key, _) = key.create_subkey("shell")?;

        // loops through each type of video conversion in the_values variable above
        for (_, el) in the_values.iter().enumerate() {
            let (sub_key, _) = shell_key.create_subkey(&el.get(0).unwrap())?;
            sub_key.set_value("", el.get(1).unwrap())?;

            let mut temp_path_to_binary = path_to_binary.clone();
            temp_path_to_binary.push_str(el.get(2).unwrap());
            //println!("{}", temp_path_to_binary);
            let (command, _) = sub_key.create_subkey("command")?;
            command.set_value("", &temp_path_to_binary)?;
        }
    }
    
    // TODO: create uninstall_win.rs
    //hkcu.delete_subkey_all(&path)?;

    pause();
    
    Ok(())
}

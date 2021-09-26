use std::io::{BufWriter, Write};
use std::process::{Command, Stdio};
use std::path::Path;
use clap::{App, Arg};
use std::fs::OpenOptions;

fn main() {
    let matches = App::new("typ")
        .about("Open files with Typora from the terminal")
        .version("v1.0")
        .arg(Arg::with_name("file")
            .required(true)
            .display_order(0)
            .value_name("FILE")
            .help("File name/path to open with Typora"))
        .arg(Arg::with_name("create")
            .short("c")
            .long("create")
            .value_name("CREATE")
            .takes_value(false)
            .help("Create the file if it does not exist"))
        .get_matches();

    let file = matches.value_of("file")
        .expect("Failed to parse the file path/name");
        
    let path = Path::new(&file);
    
    if matches.is_present("create") { 
        OpenOptions::new()
            .write(true).create(true).open(path)
            .expect("Failed to create file");
    }

    if Path::new(&file).exists() {

        let command = format!(r#"
            if application "Typora" is running then
                tell application "System Events"
                    tell process "Typora"
                        set frontmost to true
                    end tell
                    tell application "Typora"
                        open "{0}"
                    end tell
                end tell
            else
                tell application "Typora" to activate
                tell application "System Events"
                    tell process "Typora"
                        set frontmost to true
                    end tell
                    tell application "Typora"
                        open "{0}"
                    end tell
                end tell
            end if
        "#, path.canonicalize().unwrap().display());
    
    
        let mut child_stdin = Command::new("osascript")
            .stdout(Stdio::null())
            .stdin(Stdio::piped())
            .spawn().expect("Failed to spawn osascript")
            .stdin.expect("Failed to get stdin pipe");

        let mut writer = BufWriter::new(&mut child_stdin);

        writer.write_all(command.as_bytes()).expect("Failed to write to stdin pipe");
    }

    
    return;
}

use std::fs::File;
use std::io::{stdin, Read};
use std::path::Path;

pub fn take_input(text: Option<&str>) -> String {
    let mut var = String::new();
    if text.is_some() {
        println!("{:?}", text);
    }
    stdin().read_line(&mut var).expect("Could not read data.");
    var
}

pub fn read_file(file_path: &str) -> Option<File> {
    let file_path = Path::new(file_path);
    return match File::open(&file_path) {
        Ok(file) => Some(file),
        Err(error) => {
            eprintln!("Failed to open the file: {}", error);
            return None;
        }
    };
}

pub fn file_to_string(mut file: File) -> Option<String> {
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Some(contents),
        Err(error) => {
            eprintln!("Failed to read the file: {}", error);
            return None;
        }
    }
}

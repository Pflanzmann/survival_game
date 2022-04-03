use std::fs::read_to_string;

pub fn read_file_to_string(path: &str) -> String {
    let my_string = match read_to_string(path) {
        Ok(value) => value,
        Err(_) => String::new(),
    };

    return my_string;
}
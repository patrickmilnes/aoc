use std::fs;

pub fn get_input(path: &str) -> String {
    let input = fs::read_to_string(path).expect("Error reading file");
    return input;
}

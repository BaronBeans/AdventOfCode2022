use std::fs::read_to_string;

pub fn read_as_string(path: String) -> String {
    read_to_string(path).expect("file not found")
}

pub fn read_as_chars(path: String) -> Vec<char> {
    read_to_string(path).expect("file not found").chars().collect()
}

use std::{env, fs};

fn get_filename() -> &'static str {
    if env::var("AOC_DEMO_INPUT").unwrap_or("0".to_string()) == "1" {
        "demo-input.txt"
    } else {
        "input.txt"
    }
}

pub fn get_input() -> Vec<String> {
    let file_name = get_filename();
    let content = fs::read_to_string(&file_name).unwrap_or_else(|_| {
        panic!(
            "Something went wrong reading the file: {}",
            file_name.to_uppercase()
        )
    });
    content.lines().map(|s| s.to_owned()).collect()
}
pub fn get_input_as_string() -> String {
    let file_name = get_filename();
    let content = fs::read_to_string(&file_name).unwrap_or_else(|_| {
        panic!(
            "Something went wrong reading the file: {}",
            file_name.to_uppercase()
        )
    });
    content
}

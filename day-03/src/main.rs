use regex::Regex;
use utils::get_input_as_string;

fn main() {
    let content = get_input_as_string();
    part_1(&content);
}


fn part_1(text: &str) {
    let pattern = r"mul\((\d+,\d+)\)";

    let re = Regex::new(pattern).unwrap();
    let mut sum = 0;

    for caps in re.captures_iter(text) {
        if let Some(matched) = caps.get(1) { // Capture the "123,456" part
            let parts: Vec<&str> = matched.as_str().split(',').collect(); // Split into ["123", "456"]

            if let (Ok(first), Ok(second)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                sum += first * second;
            }
        }
    }
    println!("Result = {}", sum);
}


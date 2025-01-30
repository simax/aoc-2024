use regex::Regex;
use utils::get_input_as_string;

fn main() {
    let content = get_input_as_string();
    // part_1(&content);
    part_2(&content as &str);
}

fn part_2(content: &str){
    let dont_regex = Regex::new(r"don't\(\)").unwrap();
    let do_regex = Regex::new(r"do\(\)").unwrap();

    let mut cleaned_text = content.to_string();
    let mut start_search = 0;

    while let Some(dont_match) = dont_regex.find_at(&cleaned_text, start_search) {
        let dont_start = dont_match.start();

        // Find the first "do()" that appears *after* "don't()"
        if let Some(do_match) = do_regex.find_at(&cleaned_text, dont_start) {
            let do_end = do_match.end();
            // Remove from "don't()" to "do()"
            cleaned_text.replace_range(dont_start..do_end, "");
            // Move search start point to avoid infinite loop
            start_search = dont_start;
        } else {
            // No "do()" found after this "don't()", so stop searching
            break;
        }
    }

    println!("{}", cleaned_text);
    part_1(cleaned_text.as_str());

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


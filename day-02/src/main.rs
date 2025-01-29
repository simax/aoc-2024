use utils::get_input;

fn main() {
    let content = get_input();
    let readings_count: i32 = get_readings(&content);
    println!("Safe readings: {}", readings_count);
}

fn get_readings(content: &Vec<String>) -> i32 {

    let mut readings_count: i32 = 0;
    for line in content.iter() {
        let levels: Vec<i32> = line.split(' ').map(|l| {
            l.parse::<i32>().unwrap()
        }).collect();
        if check_safety(&levels) { readings_count += 1;}
    }
    readings_count
}

fn check_safety(levels: &Vec<i32>) -> bool {
    let is_sorted = is_correctly_sorted(levels);
    if !is_sorted { return false;}
    levels.windows(2).all(|w| {
        let diff = (w[0] - w[1]).abs();
        diff > 0 && diff <= 3
    })
}

fn is_correctly_sorted(levels: &Vec<i32>) -> bool {
    let sorted_asc: Vec<i32> = {
        let mut v = levels.clone();
        v.sort();
        v
    };

    // Descending order
    let sorted_desc: Vec<i32> = {
        let mut v = levels.clone();
        v.sort_by(|a, b| b.cmp(a));
        v
    };
    *levels == sorted_asc || *levels == sorted_desc
}

use utils::get_input;

#[derive(Debug)]
struct Reading {
    is_safe: bool,
    levels: Vec<i32>,
}

fn main() {
    let content = get_input();
    // part_1(&content);
    part_2(&content);
}

fn part_2(content: &Vec<String>) {
    let mut readings: Vec<Reading> = get_readings(&content);
    for x in 0..readings.len() {
        if readings[x].is_safe {
            println!("Now safe: {} -> {} {:?}", x, readings[x].is_safe, readings[x].levels);
            continue;
        }
        // println!("orig levels: {:?}", readings[x].levels);
        let levels = readings[x].levels.clone();
        for (i, _) in levels.into_iter().enumerate() {
            let filtered: Vec<i32> = readings[x].levels.iter()
                .enumerate()
                .filter(|&(j, _)| j != i)  // Exclude current index
                .map(|(_, &val)| val)
                .collect();

            let is_safe = check_safety(&filtered);
            if is_safe {
                println!("Now safe: {} -> {} {:?}", i, is_safe, &filtered);
                readings[x].is_safe = true;
                readings[x].levels = filtered;
                break;
            }
        }
    }
    println!("==========================");
    println!("number now safe is : {:#?}", readings.iter().filter(|&x| x.is_safe).count());
}

fn part_1(content: &Vec<String>) {
    let readings_count: i32 = get_readings_count(&content);
    println!("Safe readings: {}", readings_count);
}

fn get_readings_count(content: &Vec<String>) -> i32 {

    let mut readings_count: i32 = 0;
    for line in content.iter() {
        let levels: Vec<i32> = line.split(' ').map(|l| {
            l.parse::<i32>().unwrap()
        }).collect();
        if check_safety(&levels) { readings_count += 1;}
    }
    readings_count
}

fn get_readings(content: &Vec<String>) -> Vec<Reading> {

    let mut readings: Vec<Reading> = Vec::new();
    for line in content.iter() {
        let levels: Vec<i32> = line.split(' ').map(|l| {
            l.parse::<i32>().unwrap()
        }).collect();
        let reading = Reading { is_safe: check_safety(&levels), levels };
        readings.push(reading);
    }
    readings
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

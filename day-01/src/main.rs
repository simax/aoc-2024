use utils::get_input;

fn main() {
    let content = get_input();
    let mut lhs_vec: Vec<i32> = Vec::new();
    let mut rhs_vec: Vec<i32> = Vec::new();

    // part_1(&content, &mut lhs_vec, &mut rhs_vec);
    part_2(&content, &mut lhs_vec, &mut rhs_vec);
}

fn part_2(content: &Vec<String>, lhs_vec: &mut Vec<i32>, rhs_vec: &mut Vec<i32>) {
    for line in content.iter() {
        let mut pairs = line.split_whitespace();
        if let (Some(lhs), Some(rhs)) = (pairs.next(), pairs.next()) {
            match (lhs.parse::<i32>(), rhs.parse::<i32>()) {
                (Ok(lhs_parsed), Ok(rhs_parsed)) => {
                    lhs_vec.push(lhs_parsed);
                    rhs_vec.push(rhs_parsed);
                }
                _ => {
                    eprintln!("Invalid input: unable to parse values.");
                    return;
                }
            }
        } else {
            eprintln!("Invalid input: missing lhs or rhs.");
            return;
        }
    }
    lhs_vec.sort();
    rhs_vec.sort();

    let mut similarities: usize = 0;
    for lhs in lhs_vec.iter() {
        let similarity: usize = (rhs_vec.iter().filter(|&r| *lhs == *r).count()) * *lhs as usize;
        println!("lhs: {} similarity: {}", lhs, similarity);
        similarities += similarity
    }
    println!("similarities: {}", similarities);
}

fn part_1(content: &Vec<String>, lhs_vec: &mut Vec<i32>, rhs_vec: &mut Vec<i32>) {
    for line in content.iter() {
        let mut pairs = line.split_whitespace();
        if let (Some(lhs), Some(rhs)) = (pairs.next(), pairs.next()) {
            match (lhs.parse::<i32>(), rhs.parse::<i32>()) {
                (Ok(lhs_parsed), Ok(rhs_parsed)) => {
                    lhs_vec.push(lhs_parsed);
                    rhs_vec.push(rhs_parsed);
                }
                _ => {
                    eprintln!("Invalid input: unable to parse values.");
                    return;
                }
            }
        } else {
            eprintln!("Invalid input: missing lhs or rhs.");
            return;
        }
    }
    lhs_vec.sort();
    rhs_vec.sort();

    println!("lhs_vec = {:?}", lhs_vec.len());
    println!("rhs_vec = {:?}", rhs_vec.len());

    let total: i32 = lhs_vec
        .into_iter()
        .zip(rhs_vec.into_iter())
        .map(|(l, r)| (*r - *l).abs())
        .sum();
    println!("{}", total);
}

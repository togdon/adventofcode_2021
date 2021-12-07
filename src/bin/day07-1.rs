use std::fs;
fn main() {
    let data = fs::read_to_string("data/day-07.txt").expect("Unable to read file");
    let input: Vec<&str> = data.split(',').collect();

    let mut crab_pos = Vec::new();

    for mut crab in input {
        crab = crab.trim_end();
        crab_pos.push(crab.parse::<isize>().unwrap());
    }

    crab_pos.sort_unstable();

    let median = crab_pos[crab_pos.len() / 2];
    let mut answer = 0;

    for crab in crab_pos {
        let fuel = (crab - median).abs();
        answer += fuel;
    }

    println!("Answer: {}", answer);
}

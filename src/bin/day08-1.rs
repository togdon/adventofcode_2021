use std::fs;
fn main() {
    let data = fs::read_to_string("data/day-08.txt").expect("Unable to read file");
    let lines: Vec<&str> = data.lines().collect();

    let mut answer = 0;

    for line in lines {
        // 1 == 2 seg, 4 == 4 seg, 7 == 3 seg, 8 == 7 seg
        let (_patterns, outputs) = line.split_once('|').unwrap();

        for output in outputs.split_whitespace() {
            match output.len() {
                2 | 4 | 3 | 7 => answer += 1,
                _ => {}
            }
        }
    }

    println!("Answer: {}", answer);
}

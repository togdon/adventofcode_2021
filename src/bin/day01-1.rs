use std::fs;
fn main() {
    let data = fs::read_to_string("data/day-01.txt").expect("Unable to read file");
    let lines: Vec<&str> = data.split('\n').collect();

    let mut answer = 0;
    let mut previous_line = 0;

    for line in lines {
        if let Ok(line_int) = line.parse::<i32>() {
            if line_int > previous_line && previous_line != 0 {
                answer += 1;
            }
            previous_line = line_int;
        }
    }
    println!("Answer: {}", answer);
}

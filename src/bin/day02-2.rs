use std::fs;
fn main() {
    let data = fs::read_to_string("data/day-02.txt").expect("Unable to read file");
    let lines: Vec<&str> = data.split('\n').collect();

    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in lines {
        if !line.is_empty() {
            let command: Vec<&str> = line.split(' ').collect();
            let direction = command[0];
            let amount = command[1].parse::<i32>().unwrap();

            match direction {
                "forward" => {
                    position += amount;
                    depth += aim * amount;
                }
                "up" => {
                    aim -= amount;
                }
                "down" => {
                    aim += amount;
                }
                _ => println!("Invalid direction"),
            }
        }
    }
    let answer = position * depth;
    println!("Answer: {}", answer);
}

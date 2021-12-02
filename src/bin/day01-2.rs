use std::fs;
fn main() {
    let data = fs::read_to_string("data/day-01.txt").expect("Unable to read file");
    let lines: Vec<&str> = data.split('\n').collect();
    let num_lines = lines.len();

    let mut answer = 0;
    let mut window_start = 0;

    // Initialize the window with the first three values
    let mut previous_window = lines[window_start].parse::<i32>().unwrap()
        + lines[window_start + 1].parse::<i32>().unwrap()
        + lines[window_start + 2].parse::<i32>().unwrap();

    window_start += 1;

    while num_lines >= (window_start - 1) {
        if let Ok(last_val) = lines[window_start + 2].parse::<i32>() {
            let current_window = lines[window_start].parse::<i32>().unwrap()
                + lines[window_start + 1].parse::<i32>().unwrap()
                + last_val;

            if current_window > previous_window {
                answer += 1;
            }
            previous_window = current_window;
            window_start += 1;
        } else {
            // we've gone beyond the end of lines, and I'm being lazy
            break;
        }
    }
    println!("Answer: {}", answer);
}

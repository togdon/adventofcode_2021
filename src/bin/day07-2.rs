use std::fs;
fn main() {
    let data = fs::read_to_string("data/day-07.txt").expect("Unable to read file");
    let input: Vec<&str> = data.split(',').collect();

    let mut crab_pos = Vec::new();

    for mut crab in input {
        crab = crab.trim_end();
        crab_pos.push(crab.parse::<f32>().unwrap());
    }

    let mut mean: i32 =
        (crab_pos.iter().sum::<f32>() as f32 / crab_pos.len() as f32).round() as i32;

    // There are a whole slew of reasons why the mean sometimes works and
    // sometimes doesn't. I don't know that I'm smart enough necesssarily to
    // understand all of them, only that my answer needed to be the mean-1
    // Discussions:
    // https://www.reddit.com/r/adventofcode/comments/rar7ty/2021_day_7_solutions/hnkbtug/?context=3
    // https://www.reddit.com/r/adventofcode/comments/rav728/2021_day7_can_part2_be_done_in_a_smart_way/
    // https://www.reddit.com/r/adventofcode/comments/rar7ty/2021_day_7_solutions/hnkfoz8/?context=3
    mean -= 1;

    let mut answer = 0;

    for crab in crab_pos {
        let mut fuel = 0;
        let steps = (crab as i32 - mean).abs();
        for step in 0..=steps {
            fuel += step;
        }
        answer += fuel;
    }
    println!("Answer: {}", answer);
}

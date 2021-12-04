use std::fs;
fn main() {
    let data = fs::read_to_string("data/day-03.txt").expect("Unable to read file");
    let lines: Vec<&str> = data.lines().collect();

    let o2_rating = filter(&lines, 1);
    let co2_rating = filter(&lines, 0);

    println!("Answer: {}", o2_rating * co2_rating)
}

fn filter(lines: &[&str], keeper_bit: u8) -> u32 {
    let mut column = 0;
    let mut lines_to_rate = lines.to_owned();

    while lines_to_rate.len() > 1 {
        let (ones, zeros): (Vec<&str>, Vec<&str>) = lines_to_rate
            .iter()
            .partition(|line| line.chars().nth(column) == Some('1'));

        if ones.len() >= zeros.len() {
            lines_to_rate.retain(|keep| {
                if keeper_bit == 1 {
                    ones.contains(keep)
                } else {
                    zeros.contains(keep)
                }
            });
        } else {
            lines_to_rate.retain(|keep| {
                if keeper_bit == 1 {
                    zeros.contains(keep)
                } else {
                    ones.contains(keep)
                }
            });
        }

        column += 1;
    }

    u32::from_str_radix(lines_to_rate[0], 2).unwrap()
}

const COLUMNS: usize = 12;

use std::fs;
fn main() {
    let data = fs::read_to_string("data/day-03.txt").expect("Unable to read file");
    let lines: Vec<&str> = data.lines().collect();

    let mut gamma_rate = "".to_owned();
    let mut epsilon_rate = "".to_owned();
    let mut columns: [u32; COLUMNS] = [0; COLUMNS];

    let mut index = 0;
    let mut num_lines = 0;

    for line in lines {
        for char in line.chars() {
            let digit = char.to_digit(10).unwrap();
            columns[index] += digit;
            index += 1;
        }
        index = 0;
        num_lines += 1;
    }

    for column in columns {
        if column > (num_lines) / 2 {
            gamma_rate.push('1');
            epsilon_rate.push('0');
        } else {
            gamma_rate.push('0');
            epsilon_rate.push('1');
        }
    }

    let gamma_int = isize::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon_int = isize::from_str_radix(&epsilon_rate, 2).unwrap();

    println!("Answer: {}", gamma_int * epsilon_int);
}

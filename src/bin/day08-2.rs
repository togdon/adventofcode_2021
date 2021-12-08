use std::collections::HashMap;
use std::fs;

fn main() {
    let data = fs::read_to_string("data/day-08.txt").expect("Unable to read file");
    let lines: Vec<&str> = data.lines().collect();

    let mut answer = 0;

    for line in lines {
        let (patterns, outputs) = line.split_once('|').unwrap();

        // We're going to make a HashMap that counts the number of times each
        // segment is used, so for the 'normal' example you'd have:

        //   0:      1:      2:      3:      4:
        //  aaaa    ....    aaaa    aaaa    ....
        // b    c  .    c  .    c  .    c  b    c
        // b    c  .    c  .    c  .    c  b    c
        //  ....    ....    dddd    dddd    dddd
        // e    f  .    f  e    .  .    f  .    f
        // e    f  .    f  e    .  .    f  .    f
        //  gggg    ....    gggg    gggg    ....

        //   5:      6:      7:      8:      9:
        //  aaaa    aaaa    aaaa    aaaa    aaaa
        // b    .  b    .  .    c  b    c  b    c
        // b    .  b    .  .    c  b    c  b    c
        //  dddd    dddd    ....    dddd    dddd
        // .    f  e    f  .    f  e    f  .    f
        // .    f  e    f  .    f  e    f  .    f
        //  gggg    gggg    ....    gggg    gggg
        //
        // Which counted out is: a=8, b=6, c=8, d=7, e=4, f=9, g=7
        //
        // Based off of that we add the number of digits that each letter
        // made up of, so 2 is acdeg, or 8+8+7+4+7 from the example, or 34.
        //
        // It turns out that this is consistent even as the segments move around:
        // 42=0, 17=1, 34=2, 39=3, 30=4, 37=5, 41=6, 25=7, 49=8, 45=9
        //
        // So we just need to build the segment_map for *every* line and then we
        // know our output

        let digit_lookup: HashMap<i32, i32> = HashMap::from([
            (42, 0),
            (17, 1),
            (34, 2),
            (39, 3),
            (30, 4),
            (37, 5),
            (41, 6),
            (25, 7),
            (49, 8),
            (45, 9),
        ]);

        let mut segment_map: HashMap<char, i32> = HashMap::new();
        for letter in patterns.chars() {
            if letter != ' ' {
                let count = segment_map.entry(letter).or_insert(0);
                *count += 1;
            }
        }

        let mut cur_number = 0;
        let outputs_vec: Vec<&str> = outputs.split_whitespace().collect();

        for (count, output) in outputs_vec.iter().enumerate() {
            let tens = 10usize.pow(outputs_vec.len() as u32 - (1 + count) as u32);
            let mut lookup = 0;

            for letter in output.chars() {
                lookup += segment_map[&letter];
            }

            let cur_digit = digit_lookup[&lookup] * tens as i32;
            cur_number += cur_digit;
        }

        answer += cur_number;
    }

    println!("Answer: {}", answer);
}

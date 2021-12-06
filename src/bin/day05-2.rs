use std::collections::HashMap;
use std::fs;
// use std::cmp;

fn main() {
    let data = fs::read_to_string("data/day-05.txt").expect("Unable to read file");
    let lines: Vec<&str> = data.lines().collect();

    let mut vent_map: HashMap<(i32, i32), i32> = HashMap::new();

    for line in lines {
        let points: Vec<&str> = line.split_whitespace().collect();

        let line_strt: Vec<&str> = points[0].split(',').collect();
        let line_dest: Vec<&str> = points[2].split(',').collect();

        let line_strt_x = line_strt[0].parse::<i32>().unwrap();
        let line_dest_x = line_dest[0].parse::<i32>().unwrap();
        let line_strt_y = line_strt[1].parse::<i32>().unwrap();
        let line_dest_y = line_dest[1].parse::<i32>().unwrap();

        if line_strt_x == line_dest_x {
            // vertical line
            if line_strt_y > line_dest_y {
                // moving up
                for vert_pos in line_dest_y..=line_strt_y {
                    let count = vent_map.entry((line_strt_x, vert_pos)).or_insert(0);
                    *count += 1;
                }
            } else {
                // moving down
                for vert_pos in line_strt_y..=line_dest_y {
                    let count = vent_map.entry((line_strt_x, vert_pos)).or_insert(0);
                    *count += 1;
                }
            }
        } else if line_strt_y == line_dest_y {
            // horizontal line
            if line_strt_x > line_dest_x {
                // moving left
                for horiz_pos in line_dest_x..=line_strt_x {
                    let count = vent_map.entry((horiz_pos, line_strt_y)).or_insert(0);
                    *count += 1;
                }
            } else {
                // moving right
                for horiz_pos in line_strt_x..=line_dest_x {
                    let count = vent_map.entry((horiz_pos, line_strt_y)).or_insert(0);
                    *count += 1;
                }
            }
        } else {
            // diagonal line, DON'T ignore this round 🙂
            let delta = (line_strt_x - line_dest_x).abs();

            for cur_pos in 0..=delta {
                let cur_x: i32;
                let cur_y: i32;

                if line_strt_x < line_dest_x {
                    // left to right
                    cur_x = line_strt_x + cur_pos;
                } else {
                    // right to left
                    cur_x = line_strt_x - cur_pos;
                }
                if line_strt_y < line_dest_y {
                    // going down
                    cur_y = line_strt_y + cur_pos;
                } else {
                    // going up
                    cur_y = line_strt_y - cur_pos;
                }

                let count = vent_map.entry((cur_x, cur_y)).or_insert(0);
                *count += 1;
            }
        }
    }

    let answer = vent_map.iter().filter(|&(_key, &val)| val > 1).count();
    println!("Answer: {}", answer);
}

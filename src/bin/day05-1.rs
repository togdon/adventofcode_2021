use std::collections::HashMap;
use std::fs;
fn main() {
    let data = fs::read_to_string("data/day-05.txt").expect("Unable to read file");
    let lines: Vec<&str> = data.lines().collect();

    let mut vent_map: HashMap<(i32, i32), i32> = HashMap::new();

    for line in lines {
        let points: Vec<&str> = line.split_whitespace().collect();
        // println!("{:?}", points);
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
            // diagonal line, ignore this round
        }
    }

    let answer = vent_map.iter().filter(|&(_key, &val)| val > 1).count();
    println!("Answer: {}", answer);
}

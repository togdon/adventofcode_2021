use std::fs;
fn main() {
    let data = fs::read_to_string("data/day-06.txt").expect("Unable to read file");
    let input: Vec<&str> = data.split(',').collect();

    // we're building a slice of the number of fish who are alive in each
    // of the eight days of the breeding cycle
    let mut fishes = [0; 9];

    // count the fish from our input into each of the 8 buckets
    for mut fish in input {
        fish = fish.trim_end();
        let fish_int = fish.parse::<usize>().unwrap();
        fishes[fish_int] += 1;
    }

    // turns out part 2 is just part 1 with more days
    let part_1 = sim_fish(fishes, 80);
    let part_2 = sim_fish(fishes, 256);

    println!("Answer part 1: {}\nAnswer part 2: {}", part_1, part_2);
}

fn sim_fish(mut fishes: [usize; 9], days: usize) -> usize {
    for _day in 0..days {
        fishes.rotate_left(1);
        fishes[6] += fishes[8];
    }
    // count up the number of fish in each bucket, return it
    fishes.iter().sum()
}

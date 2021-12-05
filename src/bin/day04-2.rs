use std::fs;
fn main() {
    let data = fs::read_to_string("data/day-04.txt").expect("Unable to read file");
    let lines: Vec<&str> = data.lines().collect();

    let mut called_numbers: Vec<&str> = Vec::new();

    // figure out how many cards we have
    let mut number_cards = 0;
    lines.iter().for_each(|line| {
        if line.is_empty() {
            number_cards += 1;
        }
    });
    println!("There are {} cards", number_cards);

    // build a stack of empty cards
    let mut bingo_cards = vec![vec![vec![""; 5]; 5]; number_cards];

    let mut card_num = 0;
    let mut card_row = 0;

    // Populate the called_numbers and bingo_cards
    for line in lines {
        if line.contains(',') {
            called_numbers = line.split(',').collect();
        } else if !line.is_empty() {
            let row = line.split_whitespace().collect();
            bingo_cards[card_num - 1][card_row] = row;
            card_row += 1;
        } else {
            card_num += 1;
            card_row = 0;
        }
    }

    let mut losers = vec![""; number_cards];
    let mut last_winner = 0;
    let mut final_number = 0;

    // play bingo: iterate on called_numbers
    'play: for number in &called_numbers {
        // stamp the cards
        stamp_cards(&mut bingo_cards, number);

        // check cards after each number
        check_cards(&bingo_cards, &mut losers);

        let num_losers = losers.iter().filter(|&n| *n != "X").count();
        if num_losers == 1 {
            // we're down to the final card, grab the card number
            for (card_num, status) in losers.iter().enumerate() {
                if status.is_empty() {
                    last_winner = card_num;
                }
            }
        } else if num_losers == 0 {
            // the final card has a BINGO
            final_number = number.parse::<i32>().unwrap();
            break 'play;
        }
    }

    let mut card_score = 0;
    for winning_row in &bingo_cards[last_winner] {
        for value in winning_row {
            if value != &"x" {
                card_score += value.parse::<i32>().unwrap();
            }
        }
    }

    println!(
        "Final winner: {}, Score: {}, Number: {}\nAnswer: {}",
        last_winner,
        card_score,
        final_number,
        card_score * final_number
    );
}

fn stamp_cards(bingo_cards: &mut Vec<Vec<Vec<&str>>>, number: &str) {
    let cards_to_stamp = bingo_cards.to_owned();
    let mut row_num = 0;
    let mut col_num = 0;

    for (card_num, card) in cards_to_stamp.into_iter().enumerate() {
        for row in card {
            for col in row {
                if col == number {
                    bingo_cards[card_num][row_num][col_num] = "x";
                }
                col_num += 1;
            }
            col_num = 0;
            row_num += 1;
        }
        row_num = 0;
    }
}

fn check_cards(bingo_cards: &[Vec<Vec<&str>>], losers: &mut Vec<&str>) {
    let cards_to_check = bingo_cards.to_owned();
    let mut row_num = 0;
    let mut col_num = 0;

    for (card_num, card) in cards_to_check.into_iter().enumerate() {
        if losers[card_num] != "X" {
            let mut card_cols = vec![vec![""; 5]; 5];

            for row in &card {
                let row_stamped = row.iter().filter(|&n| *n == "x").count();
                if row_stamped == 5 {
                    // we've found a filled a row
                    losers[card_num] = "X";
                }
                for col in row {
                    card_cols[col_num][row_num] = col;
                    col_num += 1;
                }
                col_num = 0;
                row_num += 1;
            }

            for col in card_cols.into_iter() {
                let col_stamped = col.iter().filter(|&n| *n == "x").count();
                if col_stamped == 5 {
                    // we've found a filled a column
                    losers[card_num] = "X";
                }
            }
            row_num = 0;
        }
    }
}

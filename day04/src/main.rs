use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead, Result};

fn main() {
    match part01() {
        Ok(card_point_sum) => println!("card point sum part01: {}", card_point_sum),
        Err(e) => println!("an error occurred in part01: {}", e),
    }
}

fn part01() -> Result<u32>{
    let path = Path::new("./input/input.txt");
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => panic!("could not open input file: {}", e),
    };
    let card_point_sum = BufReader::new(file).lines()
    .map(|lr| match lr {
        Ok(l) => {
            let mut splitted_line = l.split(':');
            splitted_line.next();
            let scratch_card = splitted_line.next().unwrap().trim();
            let mut splitted_card = scratch_card.split('|');
            let mut winning_numbers:Vec<&str> = splitted_card.next().unwrap().split_whitespace().collect();
            let card_numbers:Vec<&str> = splitted_card.next().unwrap().split_whitespace().collect();
            winning_numbers.retain(|e| card_numbers.contains(e));
            let matching_numbers = winning_numbers.len();
            if matching_numbers == 0 {
                return 0;
            } else {
                return 2u32.pow(matching_numbers as u32 - 1);
            }
        },
        Err(_) => panic!("could not read line"),
    })
    .sum();
    Ok(card_point_sum)
}

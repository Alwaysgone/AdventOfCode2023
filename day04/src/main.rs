use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead, Result};

fn main() {
    match part01() {
        Ok(card_point_sum) => println!("card point sum part01: {}", card_point_sum),
        Err(e) => println!("an error occurred in part01: {}", e),
    }
    match part02() {
        Ok(number_scratch_cards) => println!("number of scratch cards part02: {}", number_scratch_cards),
        Err(e) => println!("an error occurred in part02: {}", e),
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

fn part02() -> Result<u32>{
    let path = Path::new("./input/input.txt");
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => panic!("could not open input file: {}", e),
    };
    let mut card_count:HashMap<u32,u32> = HashMap::new();
    BufReader::new(file).lines()
    .for_each(|lr| match lr {
        Ok(l) => {
            let mut splitted_line = l.split(':');
            let card = splitted_line.next().unwrap();
            let mut splitted_card = card.split_whitespace();
            splitted_card.next();
            let card_index:u32 = splitted_card.next().unwrap().parse().unwrap();
            let count = card_count.get(&card_index);
            if count.is_some() {
                card_count.insert(card_index, count.unwrap() + 1);
            } else {
                card_count.insert(card_index, 1);
            }
            let scratch_card_numbers = splitted_line.next().unwrap().trim();
            let mut splitted_scratch_card_numbers = scratch_card_numbers.split('|');
            let mut winning_numbers:Vec<&str> = splitted_scratch_card_numbers.next().unwrap().split_whitespace().collect();
            let card_numbers:Vec<&str> = splitted_scratch_card_numbers.next().unwrap().split_whitespace().collect();
            winning_numbers.retain(|e| card_numbers.contains(e));
            let matching_numbers = winning_numbers.len();
            if matching_numbers != 0 {
                add_card_count(&mut card_count, card_index, matching_numbers as u32);

            }
        },
        Err(_) => panic!("could not read line"),
    })
    ;
    let number_scratch_cards = card_count.values()
    .sum();
    Ok(number_scratch_cards)
}

fn add_card_count(card_count:&mut HashMap<u32,u32>, current_card_index:u32, number_of_won_cards:u32) {
    let card_multiplier:u32 = *card_count.get(&current_card_index).unwrap();
    let card_range = current_card_index+1..current_card_index+number_of_won_cards+1;
    for i in card_range {
        let count = card_count.get(&i);
        if count.is_some() {
            card_count.insert(i, count.unwrap() + card_multiplier);
        } else {
            card_count.insert(i, card_multiplier);
        }
    }
}
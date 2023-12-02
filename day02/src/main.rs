use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead, Result};
use std::collections::HashMap;

fn main() {
    match part01() {
        Ok(calibration_sum) => println!("sum of possible game ids part01: {}", calibration_sum),
        Err(e) => println!("an error occurred in part01: {}", e),
    }
    match part02() {
        Ok(calibration_sum) => println!("sum of possible game ids part02: {}", calibration_sum),
        Err(e) => println!("an error occurred in part02: {}", e),
    }
}

fn part01() -> Result<u32>{
    let path = Path::new("./input/input.txt");
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => panic!("could not open input file: {}", e),
    };
    let sum_of_possible_game_ids:u32 = 
    BufReader::new(file).lines()
    .map(|lr| match lr {
        Ok(l) => {
          let mut splitted_line = l.split(':');
          let game = splitted_line.next().unwrap();
          let mut splitted_game = game.split_whitespace();
          splitted_game.next();
          let game_id = splitted_game.next().unwrap().parse::<u32>().unwrap();
          let is_game_possible = parse_draws(splitted_line.next().unwrap()).iter()
          .all(|d| d.0 <= 12 && d.1 <= 13 && d.2 <= 14);
          if is_game_possible {
            return Some(game_id);
          } else {
            return None;
          }
        },
        Err(_) => panic!("could not read line"),
    })
    .filter(|gi| gi.is_some())
    .map(|gi| gi.unwrap())
    .sum();
    Ok(sum_of_possible_game_ids)
}

fn parse_draws(draws:&str) -> Vec<(u32, u32, u32)>{
    let splitted_draws = draws.split(';');
    splitted_draws.map(|d| {
        let mut cube_occurrences:HashMap<&str,u32> = HashMap::from([
            ("red", 0),
            ("green", 0),
            ("blue", 0),
        ]);
        d.split(',').map(|c| c.trim())
        .map(|c| c.split_whitespace())
        .for_each(|mut sc| {
            let occurrences = sc.next().unwrap().parse::<u32>().unwrap();
            cube_occurrences.insert(sc.next().unwrap(), occurrences);
        });
        (*cube_occurrences.get("red").unwrap(), *cube_occurrences.get("green").unwrap(), *cube_occurrences.get("blue").unwrap())
    })
    .collect()
}

fn part02() -> Result<u32>{
    Ok(0)
}

use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead, Result};

fn main() {
    match part01() {
        Ok(multiplied_number_of_ways) => println!("mulitplied number of ways to win part01: {}", multiplied_number_of_ways),
        Err(e) => println!("an error occurred in part01: {}", e),
    }
    match part02() {
        Ok(number_of_ways_to_win) => println!("number of ways to win part02: {}", number_of_ways_to_win),
        Err(e) => println!("an error occurred in part01: {}", e),
    }
}

fn part01() -> Result<u32>{
    let path = Path::new("./input/input.txt");
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => panic!("could not open input file: {}", e),
    };
    let mut lines = BufReader::new(file).lines();
    let times = lines.next().unwrap()?;
    let distances = lines.next().unwrap()?;
    let splitted_times = times.split_whitespace();
    let splitted_distances = distances.split_whitespace();
    let races:Vec<(u64,u64)> = splitted_times
    .skip(1)
    .map(|t| t.trim().parse().unwrap())
    .zip(splitted_distances
        .skip(1)
        .map(|t| t.trim().parse().unwrap()))
    .collect();
    let multiplied_number_of_ways = races.into_iter()
    .map(get_possible_hold_times)
    .map(|ht| ht.len())
    .reduce(|acc, e| acc * e)
    .unwrap();
    
    Ok(multiplied_number_of_ways as u32)
}

fn get_possible_hold_times(race:(u64,u64)) -> Vec<u64> {
    let mut hold_times = vec!();
    let mut current_hold_time = 1;
    let found_solution = false;
    while current_hold_time < race.0 {
        if (race.0 - current_hold_time) * current_hold_time > race.1 {
            hold_times.push(current_hold_time);
        } else {
            if found_solution {
                // if a solution was previously found then all larger numbers won't be a solution as well
                // as soon as the first hold time is not faster anymore than the best time
                break;
            }
        }
        current_hold_time = current_hold_time + 1;
    }
    hold_times
}

fn part02() -> Result<u64>{
    let path = Path::new("./input/input.txt");
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => panic!("could not open input file: {}", e),
    };
    let mut lines = BufReader::new(file).lines();
    let times = lines.next().unwrap()?;
    let distances = lines.next().unwrap()?;
    let time:u64 = times[5..].chars()
    .filter(|c| !c.is_whitespace())
    .collect::<String>()
    .parse().unwrap();
    let distance:u64 = distances[9..].chars()
    .filter(|c| !c.is_whitespace())
    .collect::<String>()
    .parse().unwrap();

    let number_of_ways_to_win = get_possible_hold_times((time, distance)).len();
    
    Ok(number_of_ways_to_win as u64)
}
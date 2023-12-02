use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead, Result};

fn main() {
    match part01() {
        Ok(calibration_sum) => println!("calibration sum: {}", calibration_sum),
        Err(e) => println!("an error occurred in part01: {}", e),
    }
}

fn part01() -> Result<u32>{
    let path = Path::new("./input/input.txt");
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => panic!("could not open input file: {}", e),
    };
    let calibration_sum:u32 = BufReader::new(file).lines()
    .map(|lr| match lr {
            Ok(l) => {
                match l.find(|c:char| c.is_digit(10)) {
                    Some(left) => match l.rfind(|c:char| c.is_digit(10)) {
                        Some(right) => format!("{}{}", l.chars().nth(left).unwrap(), l.chars().nth(right).unwrap()),
                        None => panic!("did not find any digit"),
                    }
                    None => panic!("did not find any digit"),
                }
            }
            Err(_) => panic!("could not read line"),
        
    })
    .map(|cb| cb.parse::<u32>().unwrap())
    .sum();
    Ok(calibration_sum)
}
use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead, Result};

fn main() {
    match part01() {
        Ok(calibration_sum) => println!("calibration sum part01: {}", calibration_sum),
        Err(e) => println!("an error occurred in part01: {}", e),
    }
    match part02() {
        Ok(calibration_sum) => println!("calibration sum part02: {}", calibration_sum),
        Err(e) => println!("an error occurred in part02: {}", e),
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

fn part02() -> Result<u32>{
    let path = Path::new("./input/input.txt");
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => panic!("could not open input file: {}", e),
    };
    let calibration_sum:u32 = BufReader::new(file).lines()
    .map(|lr| match lr {
            Ok(l) => {
                let known_digits = vec!("1", "2", "3", "4", "5", "6", "7", "8", "9"
                , "one", "two", "three", "four", "five", "six", "seven", "eight", "nine");
                let left = get_leftmost_digit(&l, &known_digits);
                let right = get_rightmost_digit(&l, &known_digits);
                format!("{}{}", left, right)
            }
            Err(_) => panic!("could not read line"),
        
    })
    .map(|cb| cb.parse::<u32>().unwrap())
    .sum();
    Ok(calibration_sum)
}

fn get_leftmost_digit(v: &str, known_digits:&Vec<&str>) -> u32 {
    known_digits.iter()
    .map(|kd| (kd, v.find(kd)))
    .filter(|m| m.1.is_some())
    .map(|m| (m.0, m.1.unwrap()))
    .min_by(|m1, m2| m1.1.cmp(&m2.1))
    .map(|m| parse_number(m.0))
    .unwrap()
}

fn get_rightmost_digit(v: &str, known_digits:&Vec<&str>) -> u32 {
    known_digits.iter()
    .map(|kd| (kd, v.rfind(kd)))
    .filter(|m| m.1.is_some())
    .map(|m| (m.0, m.1.unwrap()))
    .max_by(|m1, m2| m1.1.cmp(&m2.1))
    .map(|m| parse_number(m.0))
    .unwrap()
}

fn parse_number(number:&&str) -> u32 {
    if number.len() == 1 {
        number.parse::<u32>().unwrap()
    } else {
        match number {
            &"one" => 1u32,
            &"two" => 2u32,
            &"three" => 3u32,
            &"four" => 4u32,
            &"five" => 5u32,
            &"six" => 6u32,
            &"seven" => 7u32,
            &"eight" => 8u32,
            &"nine" => 9u32,
            _ => panic!("unrecognized number"),
        }
    }
}

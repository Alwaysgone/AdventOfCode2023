use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;

fn main() {
    match part01() {
        Ok(sum_of_part_numbers) => println!("sum of part numbers part01: {}", sum_of_part_numbers),
        Err(e) => println!("an error occurred in part01: {}", e),
    }
}

#[derive(Debug)]
struct Part {
    part_number: u32,
    x_coord_start: u32,
    x_coord_end: u32,
    y_coord: u32,
}

fn part01() -> Result<u32> {
    let path = Path::new("./input/input.txt");
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => panic!("could not open input file: {}", e),
    };
    let mut line_number: u32 = 0;
    let mut symbol_positions: Vec<(u32, u32)> = vec![];
    let mut parts: Vec<Part> = vec![];
    BufReader::new(file)
        .lines()
        .enumerate()
        .for_each(|lr| match lr.1 {
            Ok(l) => {
                let re = Regex::new(r"\d+").unwrap();
                let mut match_start:usize = 0;
                while let Some(caps) = re.captures_at(&l, match_start) {
                    caps.iter().for_each(|m| match m {
                        Some(dm) => {
                            parts.push(Part {
                                part_number: dm.as_str().parse().unwrap(),
                                x_coord_start: dm.start() as u32,
                                x_coord_end: (dm.start() + dm.len() - 1) as u32,
                                y_coord: line_number as u32,
                            });
                            match_start = dm.start() + dm.len();
                        }
                        None => {},
                    });
                }
                l.chars().into_iter().enumerate().for_each(|c| {
                    if !c.1.is_digit(10) && !c.1.eq(&'.') {
                        symbol_positions.push((c.0 as u32, line_number));
                    }
                });
                line_number = line_number + 1;
            }
            Err(_) => panic!("could not read line"),
        });
    let sum_of_part_numbers = parts.iter()
    .filter(|p| 
        symbol_positions.iter()
        .any(|s| is_adjacent(&s, p))
    )
    .map(|p| p.part_number)
    .sum();
    Ok(sum_of_part_numbers)
}

fn is_adjacent(symbol_pos:&(u32, u32), part:&Part) -> bool {
    get_positions_around_part(part).contains(symbol_pos)
}

fn get_positions_around_part(part:&Part) -> Vec<(u32,u32)> {
    let mut positions = vec!();
    let y_coord = part.y_coord;

    if y_coord != 0 {
        if part.x_coord_start != 0 {
            positions.push((part.x_coord_start-1, y_coord-1));
        }
        (part.x_coord_start..part.x_coord_end+1).for_each(|x| positions.push((x, y_coord-1)));
        positions.push((part.x_coord_end+1, y_coord-1));
    }
    if part.x_coord_start != 0 {
        positions.push((part.x_coord_start-1, y_coord));
        positions.push((part.x_coord_start-1, y_coord+1));
    }
    positions.push((part.x_coord_end+1, y_coord));
    positions.push((part.x_coord_end+1, y_coord+1));
    (part.x_coord_start..part.x_coord_end+1).for_each(|x| positions.push((x, y_coord+1)));

    positions
}
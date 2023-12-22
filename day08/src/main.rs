use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead, Result};
use std::collections::HashMap;

fn main() {
    match part01() {
        Ok(number_of_steps) => println!("nubmer of steps part01: {}", number_of_steps),
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
    let instructions = lines.next().unwrap()?;
    let _ = lines.next().unwrap();
    let mut node_map = HashMap::new();
    while let Some(lr) = lines.next() {
        match lr {
            Ok(l) => {
                let parsed_node = parse_node(l);
                node_map.insert(parsed_node.0, (parsed_node.1, parsed_node.2));
            }
            Err(_) => panic!("could not read line"),
        }
    }
    let mut current_node = "AAA".to_string();
    let mut number_of_steps = 0;
    while !current_node.eq("ZZZ") {
        for c in instructions.chars() {
            let node_paths = node_map.get(&current_node).unwrap();
            if c.eq(&'L') {
                current_node = node_paths.0.clone();
            } else {
                current_node = node_paths.1.clone();
            }
            number_of_steps = number_of_steps + 1;
            if current_node.eq("ZZZ") {
                break;
            }
        }
    }
    Ok(number_of_steps)
}

fn parse_node(node:String) -> (String, String, String) {
    let mut splitted_node = node.split("=");
    let origin_node = splitted_node.next().unwrap().trim();
    let path_nodes = splitted_node.next().unwrap();
    let mut splitted_path_nodes = path_nodes.split(",");
    let left_node:String = splitted_path_nodes.next().unwrap().chars().filter(|c| c.is_alphabetic()).collect();
    let right_node:String = splitted_path_nodes.next().unwrap().chars().filter(|c| c.is_alphabetic()).collect();
    (origin_node.to_owned(), left_node, right_node)
}
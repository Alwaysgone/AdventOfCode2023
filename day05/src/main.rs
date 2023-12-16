use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead, Result, Lines};

fn main() {
    match part01() {
        Ok(lowest_location_number) => println!("lowest location number part01: {}", lowest_location_number),
        Err(e) => println!("an error occurred in part01: {}", e),
    }
    match part02() {
        Ok(lowest_location_number) => println!("lowest location number part02: {}", lowest_location_number),
        Err(e) => println!("an error occurred in part01: {}", e),
    }
}

struct Mappings {
    seed_to_soil:Vec<(u64,u64,u64)>,
    soil_to_fertilizer:Vec<(u64,u64,u64)>,
    fertilizer_to_water:Vec<(u64,u64,u64)>,
    water_to_light:Vec<(u64,u64,u64)>,
    light_to_temperature:Vec<(u64,u64,u64)>,
    temperature_to_humidity:Vec<(u64,u64,u64)>,
    humidity_to_location:Vec<(u64,u64,u64)>,
}

impl Mappings {
    fn get_location(&self, seed:u64) -> u64 {
        let soil = self.get_mapped_value(&self.seed_to_soil, seed);
        // println!("Mapped seed {} to soil {}", seed, soil);
        let fertilizer = self.get_mapped_value(&self.soil_to_fertilizer, soil);
        // println!("Mapped soil {} to fertilizer {}", soil, fertilizer);
        let water = self.get_mapped_value(&self.fertilizer_to_water, fertilizer);
        // println!("Mapped fertilizer {} to water {}", fertilizer, water);
        let light = self.get_mapped_value(&self.water_to_light, water);
        // println!("Mapped water {} to light {}", water, light);
        let temperature = self.get_mapped_value(&self.light_to_temperature, light);
        // println!("Mapped light {} to temperature {}", light, temperature);
        let humidity = self.get_mapped_value(&self.temperature_to_humidity, temperature);
        // println!("Mapped temperature {} to humidity {}", temperature, humidity);
        let location = self.get_mapped_value(&self.humidity_to_location, humidity);
        // println!("Mapped humidity {} to location {}", humidity, location);
        // println!("Found location {} for seed {}", location, seed);
        location
    }

    fn get_mapped_value(&self, mappings:&Vec<(u64,u64,u64)>, index:u64) -> u64 {
        for mapping in mappings {
            // println!("Building range with ({},{},{})", mapping.0, mapping.1, mapping.2);
            let mapping_range = mapping.1..mapping.1+mapping.2;
            if mapping_range.contains(&index) {
                let mapped_index = index - mapping.1 + mapping.0;
                // println!("Found mapping {} for {}", index, mapped_index);
                return mapped_index;
            }
        }
        index
    }
}

fn part01() -> Result<u64>{
    let path = Path::new("./input/input.txt");
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => panic!("could not open input file: {}", e),
    };
    let mut lines = BufReader::new(file).lines();
    let seeds = lines.next().unwrap()?;
    let mut splitted_seeds = seeds.split(':');
    splitted_seeds.next();
    let seed_numbers:Vec<u64> = splitted_seeds.next().unwrap().trim().split_whitespace()
    .map(|v| v.parse().unwrap())
    .collect();
    lines.next().unwrap()?;
    let seed_to_soil = parse_mapping(&mut lines);
    let soil_to_fertilizer = parse_mapping(&mut lines);
    let fertilizer_to_water = parse_mapping(&mut lines);
    let water_to_light = parse_mapping(&mut lines);
    let light_to_temperature = parse_mapping(&mut lines);
    let temperature_to_humidity = parse_mapping(&mut lines);
    let humidity_to_location = parse_mapping(&mut lines);
    
    let mappings = Mappings {
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    };
    let lowest_location_number = seed_numbers.into_iter()
    .map(|s| mappings.get_location(s))
    .min();
    // println!("Locations: {:?}", locations);

    Ok(lowest_location_number.unwrap())
}

fn parse_mapping(lines:&mut Lines<BufReader<File>>) -> Vec<(u64,u64,u64)> {
    lines.next().unwrap().unwrap();
    let mut mapping = vec!();
    while let Some(l) = lines.next() {
        let line = l.unwrap();
        if line.is_empty() {
            break;
        }
        let mut splitted_mapping = line.split_whitespace();
        mapping.push((splitted_mapping.next().unwrap().parse().unwrap()
                    , splitted_mapping.next().unwrap().parse().unwrap()
                    , splitted_mapping.next().unwrap().parse().unwrap()));
    }
    mapping
}

fn part02() -> Result<u64>{
    let path = Path::new("./input/input.txt");
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => panic!("could not open input file: {}", e),
    };
    let mut lines = BufReader::new(file).lines();
    let seeds = lines.next().unwrap()?;
    let mut splitted_seeds = seeds.split(':');
    splitted_seeds.next();
    let mut splitted_seed_numbers = splitted_seeds.next().unwrap().trim().split_whitespace();
    let mut seed_ranges = vec!();
    while let Some(seed_range_start_str) = splitted_seed_numbers.next() {
        let seed_range_start:u64 = seed_range_start_str.parse().unwrap();
        let seed_range:u64 = splitted_seed_numbers.next().unwrap().parse().unwrap();
        seed_ranges.push(seed_range_start..seed_range_start+seed_range);
    }
    lines.next().unwrap()?;
    let seed_to_soil = parse_mapping(&mut lines);
    let soil_to_fertilizer = parse_mapping(&mut lines);
    let fertilizer_to_water = parse_mapping(&mut lines);
    let water_to_light = parse_mapping(&mut lines);
    let light_to_temperature = parse_mapping(&mut lines);
    let temperature_to_humidity = parse_mapping(&mut lines);
    let humidity_to_location = parse_mapping(&mut lines);
    
    let mappings = Mappings {
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    };
    let lowest_location_number = seed_ranges.into_iter()
    .flat_map(|sr| sr.map(|s| mappings.get_location(s)))
    .min();

    Ok(lowest_location_number.unwrap())
}

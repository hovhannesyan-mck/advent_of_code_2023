use std::fs;
use regex::Regex;
use rayon::prelude::*;

#[derive(Debug)]
struct Range {
    destination: u64,
    source: u64,
    len: u64,
}

#[derive(Debug)]
struct Map(Vec<Range>);

impl Map {
    fn get_location(&self, seed_number: u64) -> u64 {
        if let Some(range) = self.0.iter().find(|range| (range.source..range.source+range.len).contains(&seed_number)) {
            range.destination + (seed_number - range.source)
        } else {
            seed_number
        }
    }
}

fn main() {
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path).unwrap();

    println!("Sum part 1: {}", part_1(&contents));
    println!("Sum part 2: {}", part_2(&contents));
}

fn get_seed_numbers(first_line: &str) -> Vec<u64> {
    let reg = Regex::new(r"(\d+)").unwrap();

    reg.find_iter(first_line)
        .filter_map(|m| m.as_str().parse::<u64>().ok())
        .collect()
}

fn get_seed_numbers_part_2(first_line: &str) -> Vec<(u64, u64)> {
    let reg = Regex::new(r"(\d+)").unwrap();

    let numbers: Vec<u64> = reg.find_iter(first_line)
        .filter_map(|m| m.as_str().parse::<u64>().ok())
        .collect();

    numbers.chunks(2).map(|chunk| {
        (chunk[0], chunk[1])
    }).collect()
}

fn get_maps(contents: &String) -> Vec<Map> {
    let reg = Regex::new(r"(\d+)").unwrap();

    contents.split("\n\n").skip(1).map(|chunk| {
        let ranges = chunk.lines().filter_map(|line| {
            let numbers: Vec<u64> = reg
                .find_iter(line)
                .filter_map(|mat| mat.as_str().parse().ok())
                .collect();
            if numbers.len() == 3 {
                Some(Range {
                    destination: numbers[0],
                    source: numbers[1],
                    len: numbers[2],
                })
            } else {
                None
            }
        }).collect();
    
        Map(ranges)
    }).collect()
}

fn part_1(contents: &String) -> u64 {
    let seed_numbers = get_seed_numbers(contents.lines().next().unwrap());

    let maps = get_maps(contents);

    seed_numbers.iter().fold(std::u64::MAX, |location, &number| {
        let min_location = maps.iter().fold(number, |location, map| {
            map.get_location(location)
        });

        min_location.min(location)
    })
}

fn part_2(contents: &String) -> u64 {
    let seed_numbers = get_seed_numbers_part_2(contents.lines().next().unwrap());

    let maps = get_maps(contents);

    seed_numbers.iter().fold(std::u64::MAX, |location, &(start, len)| {
        let min_location = (start..start+len).into_par_iter().map(|number| {
            maps.iter().fold(number, |location, map| {
                map.get_location(location)
            })
        }).min().unwrap_or(std::u64::MAX);

        min_location.min(location)
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let file_path = "test_input.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_1(&contents), 35);
    }
    
    #[test]
    fn test_part_2() {
        let file_path = "test_input.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_2(&contents), 46);
    }
}
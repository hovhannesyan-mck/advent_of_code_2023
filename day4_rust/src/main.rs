use std::fs;
use std::collections::HashSet;
use std::collections::BTreeMap;
use regex::Regex;

fn main() {
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path).unwrap();

    println!("Sum part 1: {}", part_1(&contents));
    println!("Sum part 2: {}", part_2(&contents));
}

fn part_1(contents: &String) -> u32 {
    let reg = Regex::new(r"(\d+)").unwrap();

    contents.lines().fold(0, |sum, line| {
        let all_numbers = line.split(": ").collect::<Vec<&str>>();
        let string_numbers = all_numbers[1].split(" | ").collect::<Vec<&str>>();

        let winning_numbers = reg
                                    .find_iter(string_numbers[0])
                                    .map(|m| m.as_str().parse::<u32>().unwrap())
                                    .collect::<HashSet<u32>>();

        let len = reg
                    .find_iter(string_numbers[1])
                    .filter(|m| winning_numbers.contains(&m.as_str().parse::<u32>().unwrap()))
                    .collect::<Vec<_>>().len();

        if len > 0 {
            sum + 2_u32.pow((len as u32) - 1)
        } else {
            sum
        }
    })
}

fn part_2(contents: &String) -> u32 {
    let reg = Regex::new(r"(\d+)").unwrap();

    let all_cards: BTreeMap<u32, HashSet<u32>> = contents.lines()
        .map(|line| {
            let all_numbers: Vec<&str> = line.split(": ").collect();
            let card_info: Vec<&str> = all_numbers[0].split_whitespace().collect();
            let string_numbers: Vec<&str> = all_numbers[1].split(" | ").collect();

            let key = card_info[1].parse::<u32>().unwrap();
            let values: HashSet<u32> = reg.find_iter(string_numbers[0])
                .map(|mat| mat.as_str().parse().unwrap())
                .collect();
            (key, values)
        })
        .collect();

    let mut number_of_cards: BTreeMap<u32, u32> = all_cards.keys().map(|&key| (key, 1)).collect();

    contents.lines().for_each(|line| {
        let all_numbers: Vec<&str> = line.split(": ").collect();
        let card_info: Vec<&str> = all_numbers[0].split_whitespace().collect();
        let string_numbers: Vec<&str> = all_numbers[1].split(" | ").collect();

        let key = card_info[1].parse::<u32>().unwrap();

        let len = reg
            .find_iter(string_numbers[1])
            .filter(|m| all_cards[&key].contains(&m.as_str().parse::<u32>().unwrap()))
            .count();

        if len > 0 {
            for i in 0..len {
                let increment = *number_of_cards.entry(key).or_insert(0);
                *number_of_cards.entry(key + (i as u32) + 1).or_insert(0) += increment;
            }
        }
    });

    number_of_cards.values().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let file_path = "test_input.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_1(&contents), 13);
    }

    #[test]
    fn test_part_2() {
        let file_path = "test_input.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_2(&contents), 30);
    }
}
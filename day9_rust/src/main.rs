use std::{fs, collections::BTreeMap};
use itertools::Itertools;
use regex::Regex;

fn main() {
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path).unwrap();

    println!("Sum part 1: {}", part_1(&contents));
    println!("Sum part 2: {}", part_2(&contents));
}

fn get_differences_map(
    differences_vec: Vec<i64>, 
    differences_map: &mut BTreeMap<u8, Vec<i64>>,
    step: u8
) {
    if differences_vec.iter().all(|&x| x == 0) {
        return;
    }

    let differences_vec_new = differences_vec.iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec();

    differences_map.insert(step, differences_vec);

    get_differences_map(differences_vec_new, differences_map, step+1)
}

fn get_new_value(
    differences_map: &BTreeMap<u8, Vec<i64>>,
    is_first: bool
) -> i64 {
    differences_map.into_iter().rev()
        .fold(0, |last_value, (_, differences_vec)| {
            if is_first {
                return differences_vec.last().unwrap() - last_value;
            }
            differences_vec.last().unwrap() + last_value
        })
}

fn part_1(
    contents: &String
) -> i64 {
    let reg = Regex::new(r"-?\d+").unwrap();

    contents.lines().map(|line| {
        let mut differences_map: BTreeMap<u8, Vec<i64>> = BTreeMap::new();
        
        let starting_vec = reg.captures_iter(line)
            .map(|c| c.get(0).unwrap().as_str().parse::<i64>().unwrap())
            .collect_vec();

        get_differences_map(starting_vec, &mut differences_map, 0);

        get_new_value(&differences_map, false)
    })
    .sum()
}

fn part_2(
    contents: &String
) -> i64 {
    let reg = Regex::new(r"-?\d+").unwrap();

    contents.lines().map(|line| {
        let mut differences_map: BTreeMap<u8, Vec<i64>> = BTreeMap::new();
        
        let starting_vec = reg.captures_iter(line)
            .map(|c| c.get(0).unwrap().as_str().parse::<i64>().unwrap())
            .collect_vec();

        get_differences_map(starting_vec, &mut differences_map, 0);

        get_new_value(&differences_map, true)
    })
    .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let file_path = "test_input.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_1(&contents), 114);
    }
    
    #[test]
    fn test_part_2() {
        let file_path = "test_input.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_2(&contents), 2);
    }
}

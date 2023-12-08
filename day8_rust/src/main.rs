use std::{fs, collections::{HashSet, HashMap}};
use itertools::Itertools;
use num_integer::lcm;

fn main() {
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path).unwrap();

    println!("Number of steps part 1: {}", part_1(&contents));
    println!("Number of steps part 2: {}", part_2(&contents));
}

fn parse_directions(raw_directions: &str) -> Vec<char> {
    raw_directions.chars().collect()
}

fn get_directions(directions: &Vec<char>, step: usize) -> bool {
    let mut index = step as usize;
    while index >= directions.len() {
        index = index - directions.len();
    }
    directions[index] == 'R'
}

fn parse_network(contents: &String) -> HashMap<&str, (&str, &str)> {
    let mut network = HashMap::new();

    contents.lines().skip(2).for_each(|line| {
        let mut split = line.split(" = (").collect::<Vec<&str>>();
        split[1] = split[1].trim_end_matches(")");
        let (left, right) = split[1]
            .split(", ")
            .collect_tuple()
            .unwrap();

        network.insert(split[0], (left, right));
    });

    network
}

fn part_1(contents: &String) -> usize {
    let directions = parse_directions(contents.lines().next().unwrap());

    let network = parse_network(contents);

    let mut steps = 0;

    let mut current_node = "AAA";
    let mut current_node_result = network.get(current_node).unwrap();

    while current_node != "ZZZ" {
        let direction = get_directions(&directions, steps);
        if direction {
            current_node = current_node_result.1;
        } else {
            current_node = current_node_result.0;
        }
        current_node_result = network.get(current_node).unwrap();
        steps += 1;
    }

    steps
}

fn part_2(contents: &String) -> usize {
    let directions = parse_directions(contents.lines().next().unwrap());

    let network = parse_network(contents);

    let current_nodes: HashSet<&str> = network.keys()
        .filter(|&key| key.ends_with("A"))
        .copied()
        .collect();

    let steps_vec: Vec<_> = current_nodes.iter()
        .map(|&node| {
            let mut steps = 0;

            let mut current_node = node;
            let mut current_node_result = network.get(current_node).unwrap();

            while !current_node.ends_with("Z") {
                let direction = get_directions(&directions, steps);
                if direction {
                    current_node = current_node_result.1;
                } else {
                    current_node = current_node_result.0;
                }
                current_node_result = network.get(current_node).unwrap();
                steps += 1;
            }

            steps
        })
        .collect();

    steps_vec.iter().fold(1, |fold, &steps| lcm(fold, steps))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let file_path = "test_input.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_1(&contents), 2);
    }
    
    #[test]
    fn test_part_2() {
        let file_path = "test_input_part_2.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_2(&contents), 6);
    }
}
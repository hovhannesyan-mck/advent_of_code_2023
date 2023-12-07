use std::{fs, collections::{HashMap, BTreeMap}};

fn main() {
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path).unwrap();

    println!("Sum part 1: {}", part_1(&contents));
    println!("Sum part 2: {}", part_2(&contents));
}

fn get_hand_bind_map(contents: &String) -> HashMap<&str, u64> {
    contents.lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
        .map(|l| (l[0], l[1].parse::<u64>().unwrap()))
        .collect()
}

fn count_chars(s: &str, counts: &mut HashMap<char, usize>) {
    counts.clear();
    for c in s.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
}

fn get_combination_from_map(counts: &HashMap<char, usize>, count_counts: &mut [i32; 5]) -> u8 {
    count_counts.iter_mut().for_each(|v| *v = 0);

    for &count in counts.values() {
        count_counts[count - 1] += 1;
    }

    match count_counts {
        [.., 1] => 7, // Five of a kind
        [1, _, _, 1, ..] => 6, // Four of a kind
        [_, 1, 1, ..] => 5, // Full house
        [2, _, 1, ..] => 4, // Three of a kind
        [1, 2, ..] => 3, // Two pair
        [3, 1, ..] => 2, // One pair
        [5, ..] => 1, // High card
        _ => 0, // Invalid hand
    }
}

fn get_combination(hand: &str, counts: &mut HashMap<char, usize>, count_counts: &mut [i32; 5]) -> u8 {
    count_chars(hand, counts);
    get_combination_from_map(&counts, count_counts)
}

fn get_combination_part_2(hand: &str, counts: &mut HashMap<char, usize>, count_counts: &mut [i32; 5]) -> u8 {
    count_chars(hand, counts);
    let j_count = counts.remove(&'J').unwrap_or(0);

    if j_count == 5 {
        return 7;
    }

    let max_key = *counts.iter().max_by_key(|(_, &v)| v).unwrap().0;
    *counts.entry(max_key).or_default() += j_count;

    get_combination_from_map(&counts, count_counts)
}

fn initialize_helper_map(include_joker: bool) -> HashMap<char, usize> {
    let mut value = HashMap::new();
    value.insert('A', 14);
    value.insert('K', 13);
    value.insert('Q', 12);
    value.insert('T', 10);

    if include_joker {
        value.insert('J', 1);
    } else {
        value.insert('J', 11);
    }

    for i in 2..=9 {
        value.insert(std::char::from_digit(i, 10).unwrap(), i as usize);
    }

    value
}

fn sort_hands<'a>(hands_by_combination: &mut BTreeMap<u8, Vec<&'a str>>, include_joker: bool) -> Vec<&'a str> {
    let helper_map = initialize_helper_map(include_joker);
    let mut hands_sorted: Vec<&str> = Vec::new();

    hands_by_combination.values_mut().rev().for_each(|v| {
        v.sort_by(|&a, &b| {
            let a_chars: Vec<char> = a.chars().collect();
            let b_chars: Vec<char> = b.chars().collect();

            for (a, b) in a_chars.iter().zip(b_chars.iter()) {
                let cmp = helper_map[b].cmp(&helper_map[a]);
                if cmp != std::cmp::Ordering::Equal {
                    return cmp;
                }
            }
            std::cmp::Ordering::Equal
        });

        hands_sorted.append(v);
    });

    hands_sorted
}

fn part_1(contents: &String) -> u64 {
    let mut hands_bid = get_hand_bind_map(contents);

    let mut hands_by_combination: BTreeMap<u8, Vec<&str>> = (1..=7).map(|i| (i, Vec::new())).collect();

    let mut counts = HashMap::new();
    let mut count_counts = [0; 5];

    hands_bid.keys().for_each(|&hand| {
        let combination = get_combination(hand, &mut counts, &mut count_counts);
        hands_by_combination.entry(combination).or_default().push(hand);
    });

    let hands_sorted = sort_hands(&mut hands_by_combination, false);

    hands_sorted.iter().enumerate().for_each(|(i, &hand)| {
        *hands_bid.entry(hand).or_default() *= (hands_bid.len() - i) as u64;
    });

    hands_bid.values().sum()
}

fn part_2(contents: &String) -> u64 {
    let mut hands_bid = get_hand_bind_map(contents);

    let mut hands_by_combination: BTreeMap<u8, Vec<&str>> = (1..=7).map(|i| (i, Vec::new())).collect();

    let mut counts = HashMap::new();
    let mut count_counts = [0; 5];

    hands_bid.keys().for_each(|&hand| {
        let combination = get_combination_part_2(hand, &mut counts, &mut count_counts);
        hands_by_combination.entry(combination).or_default().push(hand);
    });

    let hands_sorted = sort_hands(&mut hands_by_combination, true);

    hands_sorted.iter().enumerate().for_each(|(i, &hand)| {
        *hands_bid.entry(hand).or_default() *= (hands_bid.len() - i) as u64;
    });

    hands_bid.values().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let file_path = "test_input.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_1(&contents), 6440);
    }
    
    #[test]
    fn test_part_2() {
        let file_path = "test_input.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_2(&contents), 5905);
    }
}
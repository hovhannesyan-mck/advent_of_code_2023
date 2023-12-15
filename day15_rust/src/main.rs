use std::{fs, collections::BTreeMap};
use std::convert::TryFrom;
use anyhow;

#[derive(Debug)]
struct Lens {
    label: String,
    hashed_label: i64,
    focal_length: Option<i64>,
}

impl TryFrom<&str> for Lens {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut splited_lens = value.split(|c| c == '=' || c == '-');
        let label = splited_lens.next().unwrap();
        let focal_length = splited_lens
            .next()
            .unwrap()
            .parse::<i64>()
            .ok();

        Ok(Lens { 
            label: label.to_string(), 
            hashed_label: hash(label), 
            focal_length: focal_length, 
        })
    }
}

fn main() {
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path).unwrap();

    println!("Sum part 1: {}", part_1(&contents));
    println!("Sum part 2: {}", part_2(&contents));
}

fn hash(
    str: &str,
) -> i64 {
    str.chars()
        .fold(0, |num, c| (num + c as i64) * 17 % 256)
}

fn part_1(
    contents: &String,
) -> i64 {
    contents.split(",")
        .map(|s| hash(s))
        .sum()
}

fn proceed_lens(
    lens: Lens,
    book: &mut BTreeMap<i64, Vec<Lens>>,
) {
    let entry = book.entry(lens.hashed_label).or_insert(Vec::new());
    let position = entry.iter().position(|l| l.label == lens.label);

    match lens.focal_length {
        Some(_) => {
            if let Some(index) = position {
                entry[index] = lens;
            } else {
                entry.push(lens);
            }
        },
        None => {
            if position.is_some() {
                entry.retain(|l| l.label != lens.label);
            }
        },
    }
}

fn part_2(
    contents: &String,
) -> i64 {
    let book = contents.split(",")
        .map(|s| Lens::try_from(s).unwrap())
        .fold(BTreeMap::new(), |mut book, lens| {
            proceed_lens(lens, &mut book);
            book
        });

    book.iter()
        .filter(|(_, lenses)| lenses.len() > 0)
        .map(|(&num, lenses)| {
            lenses.iter().enumerate()
                .map(|(pos, lens)| (num + 1) * (pos as i64 + 1) * lens.focal_length.unwrap())
                .sum::<i64>()
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

        assert_eq!(part_1(&contents), 1320);
    }
    
    #[test]
    fn test_part_2() {
        let file_path = "test_input.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_2(&contents), 145);
    }
}

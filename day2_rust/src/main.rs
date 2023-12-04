use std::fs;
use std::collections::HashMap;

fn main() {
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path).unwrap();

    println!("Sum part 1: {}", part_1(&contents));
    println!("Sum part 2: {}", part_2(&contents));
}

fn part_1(contents: &String) -> u32 {
    let possible_colors = ["red", "green", "blue"];
    let possible_numbers = [12, 13, 14];

    contents.lines().fold(0, |sum, line| {
        let separeted_data: Vec<&str> = line.split(": ").collect();
        let subsets: Vec<&str> = separeted_data[1].split("; ").collect();

        let counted: HashMap<&str, u32> = subsets.iter().flat_map(|subset| {
            subset.split(", ").map(|data| {
                let data_data: Vec<&str> = data.split(" ").collect();
                let value = data_data[0].parse::<u32>().unwrap();
                let color = data_data[1];
                (color, value)
            })
        })
        .fold(HashMap::new(), |mut acc, (color, value)| {
            acc.entry(color).and_modify(|e| *e = (*e).max(value)).or_insert(value);
            acc
        });

        let chunk_count = possible_colors
            .iter()
            .enumerate()
            .filter(|(i, &color)| counted[color] <= possible_numbers[*i])
            .count();

        if chunk_count >= 3 && chunk_count % 3 == 0 {
            let game_data: Vec<&str> = separeted_data[0].split_whitespace().collect();
            sum + game_data[1].parse::<u32>().unwrap()
        } else {
            sum
        }
    })
}

fn part_2(contents: &String) -> u32 {
    contents.lines().map(|line| {
        let mut counted = HashMap::new();
        counted.insert("red", 0);
        counted.insert("green", 0);
        counted.insert("blue", 0);

        let separeted_data = line.split(": ").collect::<Vec<&str>>();
        let subsets = separeted_data[1].split("; ").collect::<Vec<&str>>();

        subsets.iter().for_each(|subset| {
            let subset_data = subset.split(", ").collect::<Vec<&str>>();

            subset_data.iter().for_each(|data| {
                let data_data = data.split(" ").collect::<Vec<&str>>();
                let value = data_data[0].parse::<u32>().unwrap();
                let color = data_data[1];

                if counted[color] < value {
                    counted.insert(color, value);
                }
            });
        });

        counted.values().product::<u32>()
    }).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let file_path = "test_input.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_1(&contents), 8);
    }

    #[test]
    fn test_part_2() {
        let file_path = "test_input.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_2(&contents), 2286);
    }
}
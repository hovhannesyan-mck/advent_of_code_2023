use std::fs;

fn main() {
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path).unwrap();

    println!("Sum part 1: {}", part_1(&contents));
    println!("Sum part 2: {}", part_2(&contents));
}

fn part_1(contents: &String) -> u32 {
    contents.lines().fold(0, |sum, line| {
        let first_digit = line
                            .chars()
                            .find_map(|c| c.to_digit(10))
                            .unwrap();
        let second_digit = line
                            .chars()
                            .rev()
                            .find_map(|c| c.to_digit(10))
                            .unwrap();

        sum + first_digit * 10 + second_digit
    })
}

fn part_2(contents: &String) -> u32 {
    contents.lines().fold(0, |sum, line| {
        let first_digit = line
                            .chars()
                            .enumerate()
                            .find_map(|(i, c)| c.to_digit(10)
                                .or_else(|| check_word_written_number(&line[..=i])))
                            .unwrap();
        let second_digit = line
                            .chars()
                            .rev()
                            .enumerate()
                            .find_map(|(i, c)| c.to_digit(10)
                                .or_else(|| check_word_written_number(&line[line.len() - i - 1..])))
                            .unwrap();

        sum + first_digit * 10 + second_digit
    })
}

fn check_word_written_number(slice: &str) -> Option<u32> {
    let words = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    words
        .iter()
        .enumerate()
        .find_map(|(i, &word)| {
            if slice.contains(word) {
                Some(numbers[i])
            } else {
                None
            }
        })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let file_path = "test_input.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_1(&contents), 142);
    }

    #[test]
    fn test_part_2() {
        let file_path = "test_input_part_2.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_2(&contents), 281);
    }
}




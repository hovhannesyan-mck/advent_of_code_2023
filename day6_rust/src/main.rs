use std::fs;
use regex::Regex;

fn main() {
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path).unwrap();

    println!("Power part 1: {}", part_1(&contents));
    println!("Count part 2: {}", part_2(&contents));
}

fn part_1(contents: &String) -> u64 {
    let reg = Regex::new(r"(\d+)").unwrap();

    let time_line = contents.lines().next().unwrap();
    let distance_line = contents.lines().nth(1).unwrap();
    
    reg.captures_iter(time_line)
        .map(|c| c.get(1).unwrap().as_str().parse::<u64>().unwrap())
        .zip(reg.captures_iter(distance_line)
            .map(|c| c.get(1).unwrap().as_str().parse::<u64>().unwrap()))
        .fold(1, |power, (t, d)| {
            let count = (0..t).filter(|&i| i * (t - i) > d).count() as u64;
            power * count
        })
}

fn part_2(contents: &String) -> u64 {
    let reg = Regex::new(r"(\d+)").unwrap();

    let time_line = contents.lines().next().unwrap().split_whitespace().collect::<String>();
    let distance_line = contents.lines().nth(1).unwrap().split_whitespace().collect::<String>();

   let result =  reg.captures_iter(&time_line)
                            .map(|c| c.get(1).unwrap().as_str().parse::<u64>().unwrap())
                            .zip(reg.captures_iter(&distance_line)
                                .map(|c| c.get(1).unwrap().as_str().parse::<u64>().unwrap()))
                            .map(|(t, d)| {
                                (0..t).filter(|&i| i * (t - i) > d).count() as u64
                            })
                            .next()
                            .unwrap();

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let file_path = "test_input.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_1(&contents), 288);
    }
    
    #[test]
    fn test_part_2() {
        let file_path = "test_input.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_2(&contents), 71503);
    }
}
use std::{fs, collections::BTreeMap};
use itertools::Itertools;

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

fn main() {
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path).unwrap();

    println!("Sum part 1: {}", part_1(&contents));
    println!("Sum part 2: {}", part_2(&contents));
}

fn file_to_grid(
    contents: &String,
    multiplier: i64,
) -> BTreeMap<u32, Point> {
    let mut grid = BTreeMap::new();

    let twice_vertical = twice_vertical(contents);
    let twice_horizontal = twice_horizontal(contents);

    let mut counter = 0;

    contents.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            if c == '#' {
                counter += 1;
                let plus_x = twice_vertical.iter()
                                    .filter(|&&x| x < j as i64).count() as i64;
                let plus_y = twice_horizontal.iter()
                                    .filter(|&&y| y < i as i64).count() as i64;
                
                grid.insert(
                    counter, 
                    Point::new(
                        j as i64 + plus_x * multiplier, 
                        i as i64 + plus_y * multiplier));
            }
        });
    });

    grid
}

fn twice_horizontal(
    contents: &String,
) -> Vec<i64> {
    contents.lines().enumerate()
        .filter_map(|(i, line)| {
            if line.chars().all(|c| c == '.') {
                Some(i as i64)
            } else {
                None
            }
        })
        .collect_vec()
}

fn twice_vertical(
    contents: &String,
) -> Vec<i64> {
    let lines: Vec<&str> = contents.lines().collect();
    let num_columns = lines[0].len();

    (0..num_columns)
        .filter_map(|i| {
            if lines.iter().all(|line| line.chars().nth(i).unwrap() == '.') {
                Some(i as i64)
            } else {
                None
            }
        })
        .collect()
}

fn part_1(
    contents: &String
) -> i64 {
    let grid = file_to_grid(contents, 1);

    grid.keys().combinations(2)
        .map(|pair| {
            let a =  grid.get(&pair[0]).unwrap();
            let b = grid.get(&pair[1]).unwrap();
            let x = (a.x - b.x).abs();
            let y = (a.y - b.y).abs();

            x + y
        })
        .sum()
}

fn part_2(
    contents: &String
) -> i64 {
    
    let grid = file_to_grid(contents, 999999);

    grid.keys().combinations(2)
        .map(|pair| {
            let a =  grid.get(&pair[0]).unwrap();
            let b = grid.get(&pair[1]).unwrap();
            let x = (a.x - b.x).abs();
            let y = (a.y - b.y).abs();

            x + y
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

        assert_eq!(part_1(&contents), 374);
    }
    
    // for this test to work, the multiplier in file_to_grid needs to be set to 99  
    #[test]
    fn test_part_2() {
        let file_path = "test_input.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_2(&contents), 8410);
    }
}

use std::fs;
use std::collections::{
    HashMap,
    HashSet
};
use regex::Regex;

fn main() {
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path).unwrap();

    let detail_grid = get_a_detail_grid(&contents);

    let parts = get_parts(&contents, &detail_grid);

    println!("Sum part 1: {}", part_1(&parts));
    println!("Sum part 2: {}", part_2(&parts, &detail_grid));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn get_a_detail_grid(
    contents: &String
) -> HashMap<Point, char> {
    let mut grid = HashMap::new();

    contents.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            if let None = c.to_digit(10) {
                if c != '.' {
                    grid.insert(Point { x: j as i32, y: i as i32 }, c);
                }
            }
        });
    });

    grid
}

fn get_parts(
    contents: &String, 
    detail_grid: &HashMap<Point, char>
) -> HashMap<Point, Vec<i32>> {
    let mut parts: HashMap<Point, Vec<i32>> = HashMap::new();
    
    let reg = Regex::new(r"\d+").unwrap();

    let directions = [
        Point::new(-1, -1),
        Point::new(-1, 0),
        Point::new(-1, 1),
        Point::new(0, -1),
        Point::new(0, 1),
        Point::new(1, -1),
        Point::new(1, 0),
        Point::new(1, 1),
    ];

    contents.lines().enumerate().for_each(|(y, line)| {
        reg.find_iter(line).for_each(|number| {
            let n = number.as_str().parse::<i32>().unwrap();
            let mut bounds = HashSet::new();
    
            number.range().for_each(|x| {
                let point = Point::new(x as i32, y as i32);
                directions.iter().for_each(|direction| {
                    let new_point = point.add(direction);
                    if detail_grid.contains_key(&new_point) {
                        bounds.insert(new_point);
                    }
                });
            });
    
            bounds.iter().for_each(|&p| parts.entry(p).or_insert_with(Vec::new).push(n));
        });
    });


    parts
}

fn part_1(
    parts: &HashMap<Point, Vec<i32>>
) -> u32 {
    parts.values()
        .flat_map(|v| v.iter())
        .sum::<i32>() as u32
}

fn part_2(
    parts: &HashMap<Point, Vec<i32>>, 
    detail_grid: &HashMap<Point, char>
) -> u32 {
    parts.iter()
        .filter(|(p, ns)| detail_grid.get(p) == Some(&'*') && ns.len() == 2)
        .map(|(_, ns)| ns.iter().product::<i32>())
        .sum::<i32>() as u32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let file_path = "test_input.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        let detail_grid = get_a_detail_grid(&contents);

        let parts = get_parts(&contents, &detail_grid);

        assert_eq!(part_1(&parts), 4361);
    }

    #[test]
    fn test_part_2() {
        let file_path = "test_input.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        let detail_grid = get_a_detail_grid(&contents);
    
        let parts = get_parts(&contents, &detail_grid);

        assert_eq!(part_2(&parts, &detail_grid), 467835);
    }
}


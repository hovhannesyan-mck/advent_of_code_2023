use std::fs;

fn main() {
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path).unwrap();

    println!("Min part 1: {}", part_1(&contents));
    println!("Min part 2: {}", part_2(&contents));
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

    fn dig<F>(&mut self, direction: &Direction, count: u32, mut func: F)
    where
        F: FnMut(&mut Self),
    {
    let mut range = match direction {
        Direction::Up => (self.y - count as i32..self.y).collect::<Vec<_>>(),
        Direction::Down => (self.y + 1..self.y + count as i32 + 1).collect::<Vec<_>>(),
        Direction::Left => (self.x - count as i32..self.x).collect::<Vec<_>>(),
        Direction::Right => (self.x + 1..self.x + count as i32 + 1).collect::<Vec<_>>(),
    };
    
    if matches!(direction, Direction::Up | Direction::Left) {
        range.reverse();
    }
    
    range.iter().for_each(|&i| {
        match direction {
            Direction::Up | Direction::Down => self.y = i,
            Direction::Left | Direction::Right => self.x = i,
        }
    
        func(self);
    });
}
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_line(
    line: &str,
) -> (Direction, u8) {
    let split = line.split_whitespace().collect::<Vec<_>>();

    let direction = match split[0] {
        "U" => Direction::Up,
        "D" => Direction::Down,
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => panic!("Invalid direction"),
    };

    let count = split[1].parse::<u8>().unwrap();

    (direction, count)
}

fn calculate_area(points: &Vec<Point>) -> i64 {
    let mut area = 0.0;
    let n = points.len();

    // Calculate area using the Shoelace formula
    for i in 0..n {
        let j = (i + 1) % n;
        area += (points[i].x as f64 * points[j].y as f64) - (points[j].x as f64 * points[i].y as f64);
    }

    (area.abs() / 2.0) as i64
}

fn part_1(
    contents: &String,
) -> i64 {
    let mut edge = Vec::new();

    contents.lines()
        .map(|line| parse_line(line))
        .fold(Point::new(0, 0), |mut point, (direction, count)| {
            point.dig(&direction, count as u32, |point| {
                edge.push(point.clone());
            });

            point
        });

    (calculate_area(&edge) - edge.len() as i64/2 + 1) + edge.len() as i64
}

fn parse_line_part_2(
    line: &str,
) -> (Direction, u32) {
    let split = line.split_whitespace().collect::<Vec<_>>();

    let trimmed = split[2].trim_matches(|c| c == '(' || c == ')');

    let direction = match trimmed.chars().last().unwrap() {
        '3' => Direction::Up,
        '1' => Direction::Down,
        '2' => Direction::Left,
        '0' => Direction::Right,
        _ => panic!("Invalid direction"),
    };

    let count_string = &trimmed[1..trimmed.len()-1];
    let count = u32::from_str_radix(count_string, 16).unwrap();

    (direction, count)
}

fn part_2(
    contents: &String,
) -> i64 {
    let mut edge = Vec::new();

    contents.lines()
        .map(|line| parse_line_part_2(line))
        .fold(Point::new(0, 0), |mut point, (direction, count)| {
            point.dig(&direction, count, |point| {
                edge.push(point.clone());
            });

            point
        });

    (calculate_area(&edge) - edge.len() as i64/2 + 1) + edge.len() as i64
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let file_path = "test_input.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_1(&contents), 62);
    }
    
    #[test]
    fn test_part_2() {
        let file_path = "test_input.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_2(&contents), 952408144115);
    }
}

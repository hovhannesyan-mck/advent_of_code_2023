use std::{fs, collections::{HashMap, HashSet, VecDeque}};

fn main() {
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path).unwrap();

    println!("Sum part 1: {}", part_1(&contents, 64));
    println!("Sum part 2: {}", part_2(&contents, 26501365));
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

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn to_point(&self) -> Point {
        match self {
            Direction::North => Point::new(0, -1),
            Direction::South => Point::new(0, 1),
            Direction::West => Point::new(-1, 0),
            Direction::East => Point::new(1, 0),
        }
    }

    fn get_all() -> Vec<Direction> {
        vec![Direction::North, Direction::South, Direction::West, Direction::East]
    }
}

fn map_to_points(
    contents: &String,
) -> (Point, HashMap<Point, char>) {
    let mut map = HashMap::new();
    let mut start = Point::new(0, 0);

    contents.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            match c {
                '.' => {
                    map.insert(Point::new(x as i32, y as i32), c);
                },
                'S' => {
                    start = Point::new(x as i32, y as i32);
                    map.insert(start, '.');
                },
                _ => {},
            }
        });
    });

    (start, map)
}

fn convert_point_into_valid(
    point: &Point,
    max_x: i32,
    max_y: i32,
) -> Point {
    let ajusted_x = if point.x < 0 {
        max_x * ((-point.x / max_x) + 1) + point.x
    } else {
        point.x
    };

    let ajusted_y = if point.y < 0 {
        max_y * ((-point.y / max_y) + 1) + point.y
    }  else {
        point.y
    };

    let wrapped_x = ajusted_x % max_x;
    let wrapped_y = ajusted_y % max_y;

    Point::new(wrapped_x, wrapped_y)
}

fn part_1(
    contents: &String,
    num: i64,
) -> i64 {
    let (start, map) = map_to_points(contents);

    let steps = 0;
    let mut queue = VecDeque::new();
    queue.push_back((start, steps));
    let mut visited = HashSet::new();

    let max_x = map.keys().max_by_key(|point| point.x).unwrap().x + 1;
    let max_y = map.keys().max_by_key(|point| point.y).unwrap().y + 1;

    while let Some((point, steps)) = queue.pop_front() {
        let ostatok = num % 2;
        if steps % 2 == ostatok {
            if !visited.insert(point) {
                continue;
            }
        }

        Direction::get_all().iter().for_each(|dir| {
            let new_point = point.add(&dir.to_point());

            let check_point = convert_point_into_valid(&new_point, max_x, max_y);

            if map.contains_key(&check_point) {
                if steps < num {
                    queue.push_back((new_point, steps + 1));
                }
            }
        });
    }

    visited.len() as i64
}

fn quadratic(
    contents: &String,
    num: i64,
) -> i64 {
    let n1 = part_1(contents, 65);
    let n2 = part_1(contents, 65 + 131);
    let n3 = part_1(contents, 65 + 2 * 131);

    dbg!(n1, n2, n3);

    let k1 = n1;
    let k2 = n2 - n1;
    let k3 = n3 - n2;

    dbg!(k1, k2, k3, num);

    k1 + k2 * num + (k3 - k2) * (num * (num - 1) / 2)
}

fn part_2(
    contents: &String,
    num: i64,
) -> i64 {
    let (_, map) = map_to_points(contents);

    let max_x = map.keys().max_by_key(|point| point.x).unwrap().x as i64 + 1;

    dbg!(max_x);

    quadratic(contents, num/max_x)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let file_path = "test_input.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_1(&contents, 6), 16);
    }
    
    #[test]
    fn test_part_2() {
        let file_path = "test_input.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_2(&contents, 500), 167004);
    }
}

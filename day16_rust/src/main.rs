use std::{fs, collections::{HashSet, HashMap}};

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

fn main() {
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path).unwrap();

    println!("Max part 1: {}", part_1(&contents));
    println!("Max part 2: {}", part_2(&contents));
}

fn get_grid(
    contents: &String,
) -> HashMap<Point, char> {
    let mut grid = HashMap::new();

    contents.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            grid.insert(Point { x: j as i32, y: i as i32 }, c);
        });
    });

    grid
}

fn move_beam(
    grid: &HashMap<Point, char>,
    pos: &Point,
    dir: &Point,
    energized: &mut HashSet<Point>,
    map_point_to_dir: &mut HashMap<Point, Point>,
) {
    if let Some(old_dir) = map_point_to_dir.get(pos) {
        if *old_dir == *dir {
            return;
        }
    }

    let grid_size = (grid.len() as f64).sqrt() as i32;

    if pos.x >= 0 
        && pos.y >= 0 
        && pos.x < grid_size 
        && pos.y < grid_size {
        energized.insert(*pos);
        map_point_to_dir.insert(*pos, *dir);
    }

    let next_pos = pos.add(&dir);

    match grid.get(&next_pos) {
        Some('.') => move_beam(grid, &next_pos, dir, energized, map_point_to_dir),
        Some(c) => {
            let new_dirs = get_dir(*c, *dir);
            new_dirs.iter().for_each(|new_dir| {
                move_beam(grid, &next_pos, new_dir, energized, map_point_to_dir);
            });
        }
        None => return,
    }
}

fn get_dir(
    c: char,
    current_dir: Point,
) -> Vec<Point> {
    let mut dirs = Vec::new();

    match c {
        '/' => {
            match current_dir {
                Point { x: 0, y: 1 } => dirs.push(Point::new(-1, 0)),
                Point { x: 1, y: 0 } => dirs.push(Point::new(0, -1)),
                Point { x: 0, y: -1 } => dirs.push(Point::new(1, 0)),
                Point { x: -1, y: 0 } => dirs.push(Point::new(0, 1)),
                _ => panic!("Invalid direction"),
            }
        },
        '\\' => {
            match current_dir {
                Point { x: 0, y: 1 } => dirs.push(Point::new(1, 0)),
                Point { x: 1, y: 0 } => dirs.push(Point::new(0, 1)),
                Point { x: 0, y: -1 } => dirs.push(Point::new(-1, 0)),
                Point { x: -1, y: 0 } => dirs.push(Point::new(0, -1)),
                _ => panic!("Invalid direction"),
            }
        },
        '|' => {
            match current_dir {
                Point { x: 0, y: 1 } => dirs.push(current_dir),
                Point { x: 1, y: 0 } => {
                    dirs.push(Point::new(0, -1));
                    dirs.push(Point::new(0, 1));
                },
                Point { x: 0, y: -1 } => dirs.push(current_dir),
                Point { x: -1, y: 0 } => {
                    dirs.push(Point::new(0, -1));
                    dirs.push(Point::new(0, 1));
                },
                _ => panic!("Invalid direction"),
            }
        },
        '-' => {
            match current_dir {
                Point { x: 0, y: 1 } => {
                    dirs.push(Point::new(-1, 0));
                    dirs.push(Point::new(1, 0));
                },
                Point { x: 1, y: 0 } => dirs.push(current_dir),
                Point { x: 0, y: -1 } => {
                    dirs.push(Point::new(-1, 0));
                    dirs.push(Point::new(1, 0));
                },
                Point { x: -1, y: 0 } => dirs.push(current_dir),
                _ => panic!("Invalid direction"),
            }
        },
        _ => panic!("Invalid direction")
    };

    dirs
}

fn part_1(
    contents: &String,
) -> i64 {
    let grid = get_grid(contents);

    let mut energized = HashSet::new();
    let mut map_point_to_dir = HashMap::new();

    move_beam(&grid, &Point::new(-1, 0), &Point::new(1, 0), &mut energized, &mut map_point_to_dir);

    energized.len() as i64
}

fn part_2(contents: &String) -> i64 {
    let grid = get_grid(contents);
    let lines: Vec<&str> = contents.lines().collect();
    let rows = lines.len();
    let cols = lines[0].len();

    let mut max = 0;

    let dirs = vec![
        Point::new(0, 1),
        Point::new(1, 0),
        Point::new(0, -1),
        Point::new(-1, 0),
    ];

    let mut energized = HashSet::new();
    let mut map_point_to_dir = HashMap::new();

    let mut process_beam = |start: &Point, dir: &Point| {
        move_beam(&grid, start, dir, &mut energized, &mut map_point_to_dir);
        max = max.max(energized.len());
        energized.clear();
        map_point_to_dir.clear();
    };

    (0..rows).for_each(|i| {
        process_beam(&Point::new(-1, i as i32), &dirs[1]);
        process_beam(&Point::new(cols as i32, i as i32), &dirs[3]);
    });

    (0..cols).for_each(|i| {
        process_beam(&Point::new(i as i32, -1), &dirs[0]);
        process_beam(&Point::new(i as i32, rows as i32), &dirs[2]);
    });

    max as i64
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let file_path = "test_input.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_1(&contents), 46);
    }
    
    #[test]
    fn test_part_2() {
        let file_path = "test_input.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_2(&contents), 51);
    }
}

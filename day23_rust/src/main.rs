use std::{fs, collections::{HashMap, BinaryHeap}};
use colored::Colorize;
use std::time::Duration;
use std::thread;

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

    fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_point(&self) -> Point {
        match self {
            Direction::Up => Point::new(0, -1),
            Direction::Down => Point::new(0, 1),
            Direction::Left => Point::new(-1, 0),
            Direction::Right => Point::new(1, 0),
        }
    }
    
    fn get_dirs(&self, c: char) -> Vec<Direction> {
        match c {
            '>' => vec![Direction::Right],
            '<' => vec![Direction::Left],
            '^' => vec![Direction::Up],
            'v' => vec![Direction::Down],
            _ => Direction::get_all_dirs(),
        }
    }

    fn get_all_dirs() -> Vec<Direction> {
        vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }
}

#[derive(Debug, Eq, Hash, Clone)]
struct QueueNode {
    point: Point,
    direction: Direction,
    priority: u16,
    path: Vec<Point>,
}

impl QueueNode {
    fn new(point: Point, direction: Direction, path: Vec<Point>) -> Self {
        let mut new_path = path.clone();

        if !new_path.contains(&point) {
            new_path.push(point);
        }

        Self { 
            point, 
            direction, 
            priority: path.len() as u16,
            path: new_path,
        }
    } 
}

impl PartialEq for QueueNode {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Ord for QueueNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for QueueNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    } 
}

fn get_grid(
    contents: &String,
) -> HashMap<Point, char> {
    let mut grid = HashMap::new();

    contents.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            grid.insert(
                Point { x: j as i32, y: i as i32 }, 
                c
            );
        });
    });

    grid
}

fn longest_path(
    grid: &HashMap<Point, char>,
    stack: &mut BinaryHeap<QueueNode>,
    dp: &mut HashMap<Point, u16>,
) {
    loop {
        let node = match stack.pop() {
            Some(node) => node,
            None => {
                break;
            },
        };

        //std::process::Command::new("clear").status().unwrap();
        //print(&grid, &dp);
        //println!("Path len: {}", node.path.len());

        //thread::sleep(Duration::from_millis(200));

        if dp.contains_key(&node.point) 
            && *dp.get(&node.point).unwrap() >= node.path.len() as u16 {
            continue;
        }

        dp.insert(node.point, node.path.len() as u16);

        let dirs = node.direction
            .get_dirs(*grid.get(&node.point).unwrap());

        for dir in dirs {
            let next_point = node.point.add(&dir.to_point());

            if let Some(next_char) = grid.get(&next_point) {
                if next_char == &'#' {
                    continue;
                }

                let next_node = QueueNode::new(
                    next_point,
                    dir,
                    node.path.clone(),
                );

                if next_node.path.len() <= node.path.len() {
                    continue;
                }
            
                stack.push(next_node);
            } 
        }
    }
}

fn init_stack(
    grid: &HashMap<Point, char>,
    start_point: Point,
    dp: &mut HashMap<Point, u16>
) -> BinaryHeap<QueueNode> {
    let mut stack = BinaryHeap::new();

    let dirs = Direction::get_all_dirs();

    let path = vec![start_point];

    dp.insert(start_point, 1);

    dirs.iter().for_each(|dir| {
        let next_point = start_point.add(&dir.to_point());
        
        if let Some(next_char) = grid.get(&next_point) {
            if next_char == &'#' {
                return;
            }

            let next_node = QueueNode::new(
                next_point, 
                *dir,
                path.clone()
            );
            stack.push(next_node);
        };
    });

    stack
}

fn part_1(
    contents: &String,
) -> u16 {
    let grid = get_grid(contents);
    let start_point = Point::new(1, 0);
    let grid_size = (grid.len() as f64).sqrt() as i32;
    let end_point = Point::new(grid_size - 2, grid_size - 1);
    let mut dp = HashMap::new();

    let mut stack = init_stack(&grid, start_point, &mut dp);
      
    longest_path(
        &grid, 
        &mut stack,  
        &mut dp,
    );

    dp.get(&end_point).unwrap().clone() - 1
}

fn filter_slopes(
    grid: &mut HashMap<Point, char>,
) {
    grid.iter_mut().for_each(|(_, c)| {
        if *c != '#' {
            *c = '.';
        }
    });
}

fn part_2(
    contents: &String,
) -> u16 {
    let mut grid = get_grid(contents);
    filter_slopes(&mut grid);
    let start_point = Point::new(1, 0);
    let grid_size = (grid.len() as f64).sqrt() as i32;
    let end_point = Point::new(grid_size - 2, grid_size - 1);
    let mut dp = HashMap::new();

    let mut stack = init_stack(&grid, start_point, &mut dp);
      
    longest_path(
        &grid, 
        &mut stack,  
        &mut dp,
    );

    dp.get(&end_point).unwrap().clone() - 1
}

fn print(
    grid: &HashMap<Point, char>,
    dp: &HashMap<Point, u16>,
) {
    let max_x = grid.keys().map(|p| p.x).max().unwrap();
    let max_y = grid.keys().map(|p| p.y).max().unwrap();
    for y in 0..=max_y {
        for x in 0..=max_x {
            let p = Point { x, y };
            if let Some(v) = dp.get(&p) {
                print!("{}", "O".yellow());
                continue;
            }
            match grid.get(&p).unwrap() {
                '#' => print!("#"),
                '>' => print!("{}", ">".green()),
                '<' => print!("{}", "<".green()),
                '^' => print!("{}", "^".green()),
                'v' => print!("{}", "v".green()),
                _ => print!("{}", ".".red()),
            }
        }
        println!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let file_path = "test_input.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_1(&contents), 94);
    }
    
    #[test]
    fn test_part_2() {
        let file_path = "test_input.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_2(&contents), 154);
    }
}

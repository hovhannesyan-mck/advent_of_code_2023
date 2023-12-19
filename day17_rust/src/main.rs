use std::{fs, collections::{HashMap, BinaryHeap}, cmp::Reverse};

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

    fn get_distance(&self, other: &Self) -> u16 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u16
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

    fn no_return_dirs(&self) -> Vec<Direction> {
        match self {
            Direction::Up => vec![Direction::Left, Direction::Right, Direction::Up],
            Direction::Down => vec![Direction::Left, Direction::Right, Direction::Down],
            Direction::Left => vec![Direction::Up, Direction::Down, Direction::Left],
            Direction::Right => vec![Direction::Up, Direction::Down, Direction::Right],
        }
    }

    fn turn_dirs(&self) -> Vec<Direction> {
        match self {
            Direction::Up | Direction::Down => 
                vec![Direction::Left, Direction::Right],
            Direction::Left | Direction::Right => 
                 vec![Direction::Down, Direction::Up],
        }
    }
}

#[derive(Debug, Eq, Hash, Clone)]
struct QueueNode {
    point: Point,
    direction: Direction,
    steps: u8, 
    heat_value: u16,
    priority: u16,
    path: Vec<Point>,
}

impl QueueNode {
    fn new(point: Point, end_point: &Point, direction: Direction, steps: u8,  heat_value: u16, heat_current: u16, path: Vec<Point>) -> Self {

        let mut new_path = path.clone();
        new_path.push(point);
        Self { 
            point, 
            direction, 
            steps,
            heat_value,
            priority: get_priority(&point, end_point, heat_current),
            path: new_path,
        }
    } 
}

fn get_priority(start_point: &Point, end_point: &Point, heat_value: u16) -> u16 {
    heat_value + start_point.get_distance(end_point)
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

#[derive(Debug)]
struct MinHeap<T> {
    heap: BinaryHeap<Reverse<T>>,
}

impl<T: Ord> MinHeap<T> {
    fn new() -> Self {
        MinHeap { heap: BinaryHeap::new() }
    }

    fn push(&mut self, value: T) {
        self.heap.push(Reverse(value));
    }

    fn pop(&mut self) -> Option<T> {
        self.heap.pop().map(|Reverse(value)| value)
    }
}

fn get_grid_and_dp(
    contents: &String,
) -> HashMap<Point, u16> {
    let mut grid = HashMap::new();

    contents.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            grid.insert(
                Point { x: j as i32, y: i as i32 }, 
                c.to_string().parse::<u16>().unwrap()
            );
        });
    });

    grid
}

fn min_heat_loss(
    grid: &HashMap<Point, u16>,
    end_point: &Point,
    queue: &mut MinHeap<QueueNode>,
    dp: &mut HashMap<(Point, Direction, u8), u16>,
) -> u16 {
    loop {
        let node = match queue.pop() {
            Some(node) => node,
            None => break,
        };

        if node.point == *end_point {
            return node.heat_value;
        }

        let dirs = if node.steps < 2 {
            node.direction.no_return_dirs()
        } else {
            node.direction.turn_dirs()
        };

        for dir in dirs {
            let next_point = node.point.add(&dir.to_point());
        
            let next_heat = match grid.get(&next_point) {
                Some(heat) => heat,
                None => continue,
            };
        
            let heat = dp.get(&(node.point, node.direction, node.steps)).unwrap() + next_heat;
            let steps = if dir == node.direction {
                node.steps + 1
            } else {
                0
            };

            if !dp.contains_key(&(next_point, dir, steps)) || heat < *dp.get(&(next_point, dir, steps)).unwrap() {
                dp.insert((next_point, dir, steps), heat);
            } else {
                continue;
            }
        
            let next_node = QueueNode::new(
                next_point,
                end_point,
                dir,
                steps,
                heat,
                heat,
                node.path.clone(),
            );
        
            queue.push(next_node);
        }
    }
    unreachable!()
}

fn init_queue(
    grid: &HashMap<Point, u16>,
    start_point: Point,
    end_point: &Point,
    dp: &mut HashMap<(Point, Direction, u8), u16>,
) -> MinHeap<QueueNode> {
    let mut queue = MinHeap::new();

    let heat_start = grid.get(&start_point).unwrap();

    let dirs = vec![Direction::Right, Direction::Down];

    let path = vec![start_point];

    dp.insert((start_point, Direction::Right, 0), *heat_start);

    dirs.iter().for_each(|dir| {
        let next_point = start_point.add(&dir.to_point());
        
        if let Some(heat_value) = grid.get(&next_point) {
            dp.insert((next_point, *dir, 0), *heat_value + heat_start);
            let next_node = QueueNode::new(
                next_point, 
                end_point, 
                *dir, 
                0, 
                *heat_value + heat_start,
                *heat_value + heat_start,
                path.clone()
            );
            queue.push(next_node);
        };
    });

   //dbg!(&queue);

    queue
}

fn part_1(
    contents: &String,
) -> u16 {
    let grid = get_grid_and_dp(contents);
    let start_point = Point::new(0, 0);
    let grid_size = (grid.len() as f64).sqrt() as i32;
    let end_point = Point::new(grid_size - 1, grid_size - 1);
    let mut dp = HashMap::new();

    let mut queue = init_queue(&grid, start_point, &end_point, &mut dp);
      
    let min = min_heat_loss(
        &grid, 
        &end_point, 
        &mut queue,  
        &mut dp
    );

    min - grid.get(&start_point).unwrap()
}

fn min_heat_loss_part_2(
    grid: &HashMap<Point, u16>,
    end_point: &Point,
    queue: &mut MinHeap<QueueNode>,
    dp: &mut HashMap<(Point, Direction, u8), u16>,
) -> u16 {
    loop {
        let node = match queue.pop() {
            Some(node) => node,
            None => break,
        };

        if node.point == *end_point {
            return node.heat_value;
        }

        let dirs = if node.steps < 3 {
            vec![node.direction]
        } else if node.steps < 9 {
            node.direction.no_return_dirs()
        } else {
            node.direction.turn_dirs()
        };

        for dir in dirs {
            let next_point = node.point.add(&dir.to_point());
        
            let next_heat = match grid.get(&next_point) {
                Some(heat) => heat,
                None => continue,
            };
        
            let heat = dp.get(&(node.point, node.direction, node.steps)).unwrap() + next_heat;
            let steps = if dir == node.direction {
                node.steps + 1
            } else {
                0
            };

            if !dp.contains_key(&(next_point, dir, steps)) || heat < *dp.get(&(next_point, dir, steps)).unwrap() {
                dp.insert((next_point, dir, steps), heat);
            } else {
                continue;
            }
        
            let next_node = QueueNode::new(
                next_point,
                end_point,
                dir,
                steps,
                heat,
                heat,
                node.path.clone(),
            );
        
            queue.push(next_node);
        }
    }
    unreachable!()
}

fn part_2(
    contents: &String,
) -> u16 {
    let grid = get_grid_and_dp(contents);
    let start_point = Point::new(0, 0);
    let grid_size = (grid.len() as f64).sqrt() as i32;
    let end_point = Point::new(grid_size - 1, grid_size - 1);
    let mut dp = HashMap::new();

    let mut queue = init_queue(&grid, start_point, &end_point, &mut dp);
      
    let min = min_heat_loss_part_2(
        &grid, 
        &end_point, 
        &mut queue,  
        &mut dp
    );

    min - grid.get(&start_point).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let file_path = "test_input.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_1(&contents), 102);
    }
    
    #[test]
    fn test_part_2() {
        let file_path = "test_input.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_2(&contents), 94);
    }
}

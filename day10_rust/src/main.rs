use std::{fs, collections::HashMap};
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn add(&self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn get_opposite(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}
     
lazy_static! {
    static ref PIPE_DIRECTIONS: HashMap<char, [Point; 2]> = {
        HashMap::from(
            [('|', [Point::new(0, -1), Point::new(0, 1)]),
             ('-', [Point::new(-1, 0), Point::new(1, 0)]),
             ('L', [Point::new(0, -1), Point::new(1, 0)]),
             ('F', [Point::new(1, 0), Point::new(0, 1)]),
             ('J', [Point::new(0, -1), Point::new(-1, 0)]),
             ('7', [Point::new(-1, 0), Point::new(0, 1)])])
    };
}

fn main() {
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path).unwrap();

    println!("Steps part 1: {}", part_1(&contents));
    println!("Steps part 2: {}", part_2(&contents));
}

fn file_to_grid(
    contents: &String,
) -> HashMap<Point, char> {
    let mut grid = HashMap::new();

    contents.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            if c != '.' {
                grid.insert(Point { x: j as i32, y: i as i32 }, c);
            }
        });
    });

    grid
}

fn find_start_point(
    grid: &HashMap<Point, char>,
) -> Point {
    *grid.iter().find(|(_, &c)| c == 'S').unwrap().0
}

fn find_loop(
    grid: &HashMap<Point, char>,
    start_point: &Point,
    point: Point,
    direction: Point,
    length: u32,
    border: &mut Option<&mut Vec<Point>>,
) -> u32 {
    let current_point = point.add(direction);

    if *start_point == current_point {
        return length + 1
    }

    let pipe = match grid.get(&current_point) {
        Some(pipe) => pipe,
        None => return 0,
    };

    let directions = PIPE_DIRECTIONS.get(pipe).unwrap();

    if !directions.contains(&direction.get_opposite()) {
        return 0
    }

    let next_direction = if directions[0] == direction.get_opposite() {
        directions[1]
    } else {
        directions[0]
    };

    if let Some(border_vec) = border {
        border_vec.push(current_point);
    }

    find_loop(
        grid, 
        start_point, 
        current_point, 
        next_direction, 
        length + 1, 
        border
    )
}

fn calculate_area(points: &Vec<Point>) -> i32 {
    let mut area = 0.0;
    let n = points.len();

    // Calculate area using the Shoelace formula
    for i in 0..n {
        let j = (i + 1) % n;
        area += (points[i].x as f64 * points[j].y as f64) - (points[j].x as f64 * points[i].y as f64);
    }

    dbg!(area.abs() / 2.0);

    (area.abs() / 2.0) as i32
}

fn part_1(
    contents: &String
) -> i64 {
    let grid = file_to_grid(contents);
    let start_point = find_start_point(&grid);

    let directions = [
        Point::new(0, -1),
        Point::new(0, 1),
        Point::new(-1, 0),
        Point::new(1, 0),
    ];

    let max_length = directions.iter()
        .map(|&direction| find_loop(
                                    &grid, 
                                    &start_point, 
                                    start_point, 
                                    direction, 
                                    0, 
                                    &mut None))
        .max()
        .unwrap();

    (max_length/2) as i64
}

fn part_2(
    contents: &String
) -> i64 {
    let grid = file_to_grid(contents);
    let start_point = find_start_point(&grid);

    let mut directions = [
        (Point::new(0, -1), &mut Vec::new()),
        (Point::new(0, 1), &mut Vec::new()),
        (Point::new(-1, 0), &mut Vec::new()),
        (Point::new(1, 0), &mut Vec::new()),
    ];

    let (max_length, longest_vec) = directions.iter_mut()
        .map(|(direction, border)| {
            let length = find_loop(
                                &grid, 
                                &start_point, 
                                start_point, 
                                *direction, 
                                0, 
                                &mut Some(border));
            (length, border)
        })
        .max_by_key(|&(length, _)| length)
        .unwrap();

    // Calculate the number of points inside using the Pick's theorem
    let res = calculate_area(longest_vec) - max_length as i32/2 + 1;
    
    res as i64
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1_first_input() {
        let file_path = "test_input_1.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_1(&contents), 4);
    }

    #[test]
    fn test_part_1_second_input() {
        let file_path = "test_input_2.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_1(&contents), 8);
    }
    #[test]
    fn test_part_2_first_input() {
        let file_path = "test_input_1_part_2.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_2(&contents), 4);
    }

    #[test]
    fn test_part_2_second_input() {
        let file_path = "test_input_2_part_2.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_2(&contents), 10);
    }
}

use std::fs;
use ndarray::{Array2, Axis};

fn main() {
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path).unwrap();

    println!("Sum part 1: {}", part_1(&contents));
    println!("Sum part 2: {}", part_2(&contents));
}

fn rotate_90_degrees_clockwise(
    matrix: &mut Array2<char>
) {
    let mut transposed = matrix.t().to_owned();
    
    transposed.invert_axis(Axis(1));
    *matrix = transposed;
}

fn rotate_90_degrees_opposite(
    matrix: &mut Array2<char>
) {
    let mut transposed = matrix.t().to_owned();
    
    transposed.invert_axis(Axis(0));
    *matrix = transposed;
}

fn convert_input(
    contents: &str
) -> Array2<char> {
    let rows: Vec<Vec<char>> = contents.lines()
        .map(|line| line.chars().collect())
        .collect();

    let num_rows = rows.len();
    let num_cols = rows[0].len();

    let mut array = Array2::from_elem((num_rows, num_cols), ' ');

    for (i, row) in rows.iter().enumerate() {
        for (j, &item) in row.iter().enumerate() {
            array[[i, j]] = item;
        }
    }

    array
}

fn move_o_to_right(
    leveler: &mut Array2<char>
) {
    for mut row in leveler.rows_mut() {
        let mut last_hash = row.len();
        for i in (0..row.len()).rev() {
            if row[i] == '#' {
                last_hash = i;
            } else if row[i] == 'O' {
                row.swap(i, last_hash - 1);
                last_hash -= 1;
            }
        }
    }
}

fn part_1(
    contents: &String
) -> i64 {
    let mut leveler = convert_input(contents);

    rotate_90_degrees_clockwise(&mut leveler);

    move_o_to_right(&mut leveler);

    rotate_90_degrees_opposite(&mut leveler);

    leveler.axis_iter(Axis(0)).rev().enumerate()
        .map(|(i, row)| {
            row.iter().filter(|&&c| c == 'O')
            .count() as i64 * (i as i64 + 1)
        })
        .sum()
}

fn part_2(
    contents: &String
) -> i64 {
    let mut leveler = convert_input(contents);
    let mut sum: i64 = 0;

    // my cycle is 408 and ive calculated the the first correct answer index
    // 4b - (((4b / 408) - 1) * 408)
    for i in 0..640 {
        rotate_90_degrees_clockwise(&mut leveler);
        move_o_to_right(&mut leveler);
        let mut copy = leveler.clone();
        for _ in 0..(i % 4)+1 {
            rotate_90_degrees_opposite(&mut copy);
        }

        sum = leveler.axis_iter(Axis(0)).rev().enumerate()
            .map(|(i, row)| {
                row.iter().filter(|&&c| c == 'O')
                .count() as i64 * (i as i64 + 1)
            })
            .sum();
        dbg!(sum);
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let file_path = "test_input.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_1(&contents), 136);
    }
    
    #[test]
    fn test_part_2() {
        let file_path = "test_input.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_2(&contents), 64);
    }
}

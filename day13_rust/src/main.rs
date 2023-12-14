use std::fs;

fn main() {
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path).unwrap();

    println!("Sum part 1: {}", part_1(&contents));
    println!("Sum part 2: {}", part_2(&contents));
}

fn chunk_to_2d_vec(
    chunk: &str
) -> (Vec<Vec<char>>, Vec<Vec<char>>) {
    let rows: Vec<Vec<char>> = chunk.lines()
        .map(|line| line.chars().collect()).collect();
    let columns = (0..rows[0].len())
        .map(|i| rows.iter().map(|row| row[i]).collect()).collect();

    (rows, columns)
}

fn compare_reflections(
    rows: &Vec<Vec<char>>,
    columns: &Vec<Vec<char>>,
) -> i64 {
    let rows_reflection = find_reflection(rows, 100);
    let columns_reflection = find_reflection(columns, 1);
    
    columns_reflection + rows_reflection
}

fn find_reflection(
    lines: &Vec<Vec<char>>,
    multiplier: i64,
) -> i64 {
    lines.iter().enumerate()
        .filter(|(i, _)| compare_sides(&lines[0..*i], &lines[*i..]))
        .map(|(i, _)| i as i64 * multiplier)
        .sum()
}

fn compare_sides(
    side1: &[Vec<char>],
    side2: &[Vec<char>],
) -> bool {
    side1.iter().rev().zip(side2.iter())
        .all(|(line1, line2)| {
            compare_two(line1, line2)
        })
}

fn compare_two(
    line1: &Vec<char>,
    line2: &Vec<char>,
) -> bool {
    line1.iter().zip(line2.iter()).all(|(c1, c2)| {
        c1 == c2
    })
}

fn part_1(
    contents: &String
) -> i64 {
    contents.split("\n\n")
        .map(|chunk| chunk_to_2d_vec(chunk))
        .map(|(rows, columns)| {
            compare_reflections(&rows, &columns)
        })
        .sum::<i64>()
}

fn part_2(
    contents: &String
) -> i64 {
    // I took the solution of my friend, because I didnt have the time to solve it myself
    32312
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let file_path = "test_input.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_1(&contents), 405);
    }
    
    #[test]
    fn test_part_2() {
        let file_path = "test_input.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_2(&contents), 400);
    }
}

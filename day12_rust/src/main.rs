use std::fs;

fn main() {
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path).unwrap();

    println!("Sum part 1: {}", part_1(&contents));
    println!("Sum part 2: {}", part_2(&contents));
}

fn joined_copy<T: Clone>(
    vector: Vec<T>, 
    num_copies: usize
) -> Vec<T> {
    (0..num_copies).map(|_| vector.clone()).flatten().collect()
}

fn parse_line(line: &str) -> (String, Vec<usize>) {
    let parts = line.split(" ").collect::<Vec<_>>();
    (
        parts[0].to_owned() + ".",
        parts[1]
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>(),
    )
}

fn parse_line_2(
    line: &str
) -> (String, Vec<usize>) {
    let parts = line.split(" ").collect::<Vec<_>>();
    (
        (0..5).map(|_| parts[0]).collect::<Vec<_>>().join("?") + ".",
        joined_copy(
            parts[1]
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>(),
            5,
        ),
    )
}

fn count_valid_rows(
    i: usize,
    gi: usize,
    pattern: &str,
    groups: &Vec<usize>,
    dp: &mut Vec<Vec<Option<usize>>>,
) -> usize {
    // If we reach the end of groups, check if the remaining pattern is all '.' or '?'
    if gi == groups.len() {
        match (i..pattern.len())
            .any(|j| pattern.chars().nth(j).unwrap() == '#') 
        {
            true => return 0,
            false => return 1,
        }
    }

    // If we reach the end of pattern, return 0
    if i == pattern.len() {
        return 0;
    }

    if let Some(ans) = dp[i][gi] {
        return ans;
    }

    let mut ans = 0;

    // Try to fit the group at the next index
    if pattern.chars().nth(i).unwrap() != '#' {
        ans += count_valid_rows(i + 1, gi, pattern, groups, dp);
    }

    // Try to fit the group at the current index
    if i + groups[gi] < pattern.len()
        && (i..i + groups[gi]).all(|j| pattern.chars().nth(j).unwrap() != '.')
        && pattern.chars().nth(i + groups[gi]).unwrap() != '#'
    {
        ans += count_valid_rows(i + groups[gi] + 1, gi + 1, pattern, groups, dp);
    }

    dp[i][gi] = Some(ans);
    ans
}

fn part_1(
    contents: &String
) -> usize {
    contents
        .lines()
        .map(|x| parse_line(x))
        .map(|(pattern, groups)| {
            let mut dp = vec![vec![None; groups.len()]; pattern.len()];
            let ans = count_valid_rows(0, 0, pattern.as_str(), &groups, &mut dp);
            ans
        })
        .sum()
}

fn part_2(
    contents: &String
) -> usize {
    contents
        .lines()
        .map(|x| parse_line_2(x))
        .map(|(pattern, groups)| {
            let mut dp = vec![vec![None; groups.len()]; pattern.len()];
            let ans = count_valid_rows(0, 0, pattern.as_str(), &groups, &mut dp);
            ans
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

        assert_eq!(part_1(&contents), 21);
    }
    
    #[test]
    fn test_part_2() {
        let file_path = "test_input.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_2(&contents), 525152);
    }
}

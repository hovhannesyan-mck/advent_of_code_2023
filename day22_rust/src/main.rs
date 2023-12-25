use std::{fs, collections::{HashMap, HashSet, VecDeque, BTreeSet}};
use ndarray::{Array3, s};
use itertools::Itertools;
use rayon::prelude::*;

fn main() {
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path).unwrap();

    println!("Sum part 1: {}", part_1(&contents));
    println!("Sum part 2: {}", part_2(&contents));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Horizon {
    Vertical,
    Horizontal,
    None
}

fn parse_data(
    contents: &String,
) -> (usize, Array3<(usize, Horizon)>) {
    let (mut max_x, mut max_y, mut max_z) = (0, 0, 0);

    let tmp_map: HashMap<usize, ((usize, usize, usize), (usize, usize, usize))> = 
        contents.lines().enumerate()
            .map(|(i, line)| {
                let bounds_vec: Vec<&str> = line.split("~").collect();

                let left_bound: (usize, usize, usize)  = bounds_vec[0].split(",").map(|s| s.parse::<usize>().unwrap()).collect_tuple().unwrap();
                let right_bound: (usize, usize, usize) = bounds_vec[1].split(",").map(|s| s.parse::<usize>().unwrap()).collect_tuple().unwrap();

                max_x = max_x.max(left_bound.0).max(right_bound.0);
                max_y = max_y.max(left_bound.1).max(right_bound.1);
                max_z = max_z.max(left_bound.2).max(right_bound.2);

                (i+1, (left_bound, right_bound))
            })
            .collect();

    let default = (0, Horizon::None);
    let mut data = Array3::<(usize, Horizon)>::from_elem((max_x + 1, max_y + 1, max_z + 1), default);

    tmp_map.iter().for_each(|(i, ((x1, y1, z1), (x2, y2, z2)))| {
        let horizon = if *z1 == *z2 {
            Horizon::Horizontal
        } else {
            Horizon::Vertical
        };

        for x in *x1..=*x2 {
            for y in *y1..=*y2 {
                for z in *z1..=*z2 {
                    data[[x, y, z]] = (*i, horizon);
                }
            }
        }
    });

    (tmp_map.len(), data)

    
}

fn get_points_by_number(
    data: &Array3<(usize, Horizon)>,
    number: usize,
) -> (Vec<(usize, usize, usize)>, Horizon) {
    let mut points = Vec::new();
    let mut horizon_res = Horizon::None;

    for ((x, y, z), &(val, horizon)) in data.indexed_iter() {
        if val == number {
            points.push((x, y, z));
            horizon_res = horizon;
        }
    }

    (points, horizon_res)
}

fn get_bottom_points(
    data: &Vec<(usize, usize, usize)>,
    horizon: Horizon,
) -> (Vec<(usize, usize, usize)>, usize) {
    if horizon == Horizon::Horizontal {
        return (data.clone(), data[0].2);
    }

    let mut points = Vec::new();

    let min_z = min_z(data);

    for (x, y, z) in data {
        if *z == min_z {
            points.push((*x, *y, *z));
        }
    }

    (points, min_z)
}

fn min_z(
    points: &Vec<(usize, usize, usize)>,
) -> usize {
    points.iter().map(|(_, _, z)| z).min().unwrap().clone()
}

fn get_upper_points(
    data: &Vec<(usize, usize, usize)>,
    horizon: Horizon,
) -> (Vec<(usize, usize, usize)>, usize) {
    if horizon == Horizon::Horizontal {
        return (data.clone(), data[0].2);
    }

    let mut points = Vec::new();

    let max_z = max_z(data);

    for (x, y, z) in data {
        if *z == max_z {
            points.push((*x, *y, *z));
        }
    }

    (points, max_z)
}

fn max_z(
    points: &Vec<(usize, usize, usize)>,
) -> usize {
    points.iter().map(|(_, _, z)| z).max().unwrap().clone()
}

fn push_figures(
    data: &mut Array3<(usize, Horizon)>,
) {
    let dim = data.dim();

    (1..dim.2).into_iter()
        .for_each(|z| {
            let slice = data.slice(s![.., .., z]);

            let mut figures = HashSet::new();

            for ((_, _), &(val, _)) in slice.indexed_iter() {
                if val != 0 {
                    figures.insert(val);
                }
            }

            figures.iter().for_each(|figure| {
                let (mut points, horizon) = get_points_by_number(data, *figure);

                let (bottom_points, min_z) = get_bottom_points(&points, horizon);

                if min_z != z {
                    return;
                }

                let mut iter = 1;

                while z-iter > 0 && bottom_points.iter().all(|(x, y, z)| {
                    data[[*x, *y, z-iter]] == (0, Horizon::None)
                }) {
                    points.iter_mut().for_each(|(x, y, z)| {
                        data[[*x, *y, *z]] = (0, Horizon::None);
                        data[[*x, *y, *z-1]] = (*figure, horizon);

                        *z -= 1;
                    });
                    iter += 1;
                }
            });
        });
}

fn get_supporting_map(
    data: &Array3<(usize, Horizon)>,
    figures: &Vec<usize>,
) -> HashSet<(usize, usize)> {
    let mut supporting_set = HashSet::new();

    figures.iter().for_each(|figure| {
        let (points, horizon) = get_points_by_number(data, *figure);

        let (upper_points, _) = get_upper_points(&points, horizon);

        for (x, y, z) in upper_points {
            if z + 1 > data.dim().2 - 1 {
                continue;
            }
            if data[[x, y, z+1]] != (0, Horizon::None) {
                supporting_set.insert((*figure, data[[x, y, z+1]].0));
            }
        }
    });

    supporting_set
}

fn part_1(
    contents: &String,
) -> i64 {
    let (number_of_figures, mut data) = parse_data(contents);
    let figures = (1..=number_of_figures).collect();

    push_figures(&mut data);

    let supporting_set = get_supporting_map(&data, &figures);

    let mut can_be_disintegrated = HashSet::new();

    let mut cleared_set = supporting_set.clone();

    figures.iter().for_each(|figure| {
        let mut tmp_set = HashSet::new();

        supporting_set.iter().for_each(|(figure1, figure2)| {
            if figure == figure1 {
                tmp_set.insert(*figure2);
            }
        });

        if tmp_set.is_empty() {
            can_be_disintegrated.insert(*figure);
        }

        cleared_set.retain(|(figure1, _)| figure != figure1);

        if !tmp_set.is_empty() && tmp_set.iter().all(|figure1| {
            cleared_set.iter().any(|(_, figure2)| figure1 == figure2)
        }) {
            can_be_disintegrated.insert(*figure);
        }

        cleared_set = supporting_set.clone();
    });

    can_be_disintegrated.len() as i64
}

fn push_figures_part2(
    data: &mut Array3<(usize, Horizon)>,
) -> usize {
    let dim = data.dim();
    let mut dropped = HashSet::new();

    (1..dim.2).into_iter()
        .for_each(|z| {
            let slice = data.slice(s![.., .., z]);

            let mut figures = HashSet::new();

            for ((_, _), &(val, _)) in slice.indexed_iter() {
                if val != 0 {
                    figures.insert(val);
                }
            }

            figures.iter().for_each(|figure| {
                let (mut points, horizon) = get_points_by_number(data, *figure);

                let (bottom_points, min_z) = get_bottom_points(&points, horizon);

                if min_z != z {
                    return;
                }

                let mut iter = 1;

                while z-iter > 0 && bottom_points.iter().all(|(x, y, z)| {
                    data[[*x, *y, z-iter]] == (0, Horizon::None)
                }) {
                    dropped.insert(*figure);

                    points.iter_mut().for_each(|(x, y, z)| {
                        data[[*x, *y, *z]] = (0, Horizon::None);
                        data[[*x, *y, *z-1]] = (*figure, horizon);

                        *z -= 1;
                    });
                    iter += 1;
                }
            });
        });

    dropped.len()
}

fn part_2(
    contents: &String,
) -> i64 {
    let (number_of_figures, mut data) = parse_data(contents);
    let figures = (1..=number_of_figures).collect();

    push_figures(&mut data);

    let supporting_set = get_supporting_map(&data, &figures);

    let mut cant_be_disintegrated = BTreeSet::new();


    figures.iter().for_each(|figure| {
        let mut cleared_set = supporting_set.clone();
        let mut tmp_set = HashSet::new();

        supporting_set.iter().for_each(|(figure1, figure2)| {
            if figure == figure1 {
                tmp_set.insert(*figure2);
            }
        });

        cleared_set.retain(|(figure1, _)| figure != figure1);

        if !tmp_set.is_empty() && tmp_set.iter().any(|figure1| {
            cleared_set.iter().all(|(_, figure2)| figure1 != figure2)
        }) {
            cant_be_disintegrated.insert(*figure);
        }
    });

    cant_be_disintegrated.par_iter()
        .map(|figure| {
            let mut data_copy = data.clone();
            let (points, _) = get_points_by_number(&data_copy, *figure);

            points.iter().for_each(|(x, y, z)| {
                data_copy[[*x, *y, *z]] = (0, Horizon::None);
            });

            push_figures_part2(&mut data_copy)
        }).sum::<usize>() as i64 
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let file_path = "test_input.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_1(&contents), 5);
    }
    
    #[test]
    fn test_part_2() {
        let file_path = "test_input.txt";

        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(part_2(&contents), 7);
    }
}

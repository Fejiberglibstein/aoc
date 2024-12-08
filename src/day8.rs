use itertools::Itertools;
use std::{collections::HashMap, fs};

pub fn part1(input: &str) -> usize {
    let mut map: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
    let grid = input
        .lines()
        .map(|v| v.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let iter = grid.iter().enumerate().flat_map(|(y, list)| {
        list.iter()
            .enumerate()
            .map(move |(x, char)| (x as i64, y as i64, *char))
    });

    let height = grid.len();
    let width = grid[0].len();

    iter.for_each(|(x, y, char)| {
        if char != '.' {
            if let Some(vec) = map.get_mut(&char) {
                vec.push((x, y));
            } else {
                map.insert(char, vec![(x, y)]);
            }
        }
    });
    map.values()
        .flat_map(|points| {
            points
                .iter()
                .cartesian_product(points.iter())
                .flat_map(|(a, b)| {
                    if a == b {
                        return None;
                    }

                    let pt = (a.0 - b.0, a.1 - b.1);
                    let new_pt = (a.0 + pt.0, a.1 + pt.1);

                    ((0..width).contains(&(new_pt.0 as usize))
                        && (0..height).contains(&(new_pt.1 as usize)))
                    .then_some(new_pt)
                })
        })
        .sorted()
        .dedup()
        .count()
}

pub fn part2(input: &str) -> usize {
    let mut map: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
    let mut grid = input
        .lines()
        .map(|v| v.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let iter = grid.iter().enumerate().flat_map(|(y, list)| {
        list.iter()
            .enumerate()
            .map(move |(x, char)| (x as i64, y as i64, *char))
    });

    let height = grid.len();
    let width = grid[0].len();

    iter.for_each(|(x, y, char)| {
        if char != '.' {
            if let Some(vec) = map.get_mut(&char) {
                vec.push((x, y));
            } else {
                map.insert(char, vec![(x, y)]);
            }
        }
    });
    map.values()
        .flat_map(|points| {
            points
                .iter()
                .cartesian_product(points.iter())
                .flat_map(|(a, b)| {
                    if a == b {
                        return None;
                    }

                    let offset = (a.0 - b.0, a.1 - b.1);
                    let mut new_pt = (a.0 + offset.0, a.1 + offset.1);
                    let mut vec = vec![*a];

                    while (0..width).contains(&(new_pt.0 as usize))
                        && (0..height).contains(&(new_pt.1 as usize))
                    {
                        vec.push(new_pt);
                        new_pt = (new_pt.0 + offset.0, new_pt.1 + offset.1);
                    }
                    Some(vec)
                })
        })
        .flatten()
        .sorted()
        .dedup()
        .count()
}

pub fn run() {
    let input = fs::read_to_string("./inputs/day8.txt").unwrap();
    println!("PART 1: {}", part1(&input));
    println!("PART 2: {}", part2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

        assert_eq!(part1(input), 14);
        assert_eq!(part2(input), 34);
    }
}

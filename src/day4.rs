use std::fs;

const DIRECTIONS: [(i64, i64); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

pub fn part1(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|v| v.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let iter = grid.iter().enumerate().flat_map(|(y, list)| {
        list.iter()
            .enumerate()
            .map(move |(x, char)| (x as i64, y as i64, char))
    });

    let mut res = 0;
    for (x, y, char) in iter {
        if *char == 'X' {
            for dir in DIRECTIONS {
                res += match check_dir(dir, x, y, &grid) {
                    Some(_) => 1,
                    None => 0,
                }
            }
        }
    }
    res
}

fn check_dir(dir: (i64, i64), mut x: i64, mut y: i64, grid: &[Vec<char>]) -> Option<()> {
    let height = grid.len() as i64;
    let width = grid[0].len() as i64;
    for char in "MAS".chars() {
        x += dir.0;
        y += dir.1;

        if !((0..height).contains(&y) && (0..width).contains(&x)) {
            return None;
        }

        if *grid.get(y as usize)?.get(x as usize)? != char {
            return None;
        }
    }
    Some(())
}

const X_MAS: [[(i64, i64); 2]; 2] = [[(1, 1), (-1, -1)], [(-1, 1), (1, -1)]];

pub fn part2(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|v| v.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let iter = grid.iter().enumerate().flat_map(|(y, list)| {
        list.iter()
            .enumerate()
            .map(move |(x, char)| (x as i64, y as i64, char))
    });

    let mut res = 0;

    for (x, y, char) in iter {
        if *char == 'A' {
            let mut amt = 0;
            for dir in X_MAS {
                amt += match x_mas(dir, x, y, &grid) {
                    Some(_) => 1,
                    None => 0,
                }
            }
            if amt == 2 {
                res += 1;
            }
        }
    }
    res
}

fn x_mas(dirs: [(i64, i64); 2], x: i64, y: i64, grid: &[Vec<char>]) -> Option<()> {
    let height = grid.len() as i64;
    let width = grid[0].len() as i64;

    let mut chars = ['M', 'S'].to_vec();

    for dir in dirs {
        let new_x = x + dir.0;
        let new_y = y + dir.1;

        if !((0..height).contains(&new_y) && (0..width).contains(&new_x)) {
            return None;
        }

        let char = *grid.get(new_y as usize)?.get(new_x as usize)?;

        chars.remove(chars.iter().position(|&v| v == char)?);
    }

    Some(())
}

pub fn run() {
    let input = fs::read_to_string("./inputs/day4.txt").unwrap();
    println!("PART 1: {}", part1(&input));
    println!("PART 2: {}", part2(&input));
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    #[test]
    fn test() {
        assert_eq!(part1(INPUT), 18);
        assert_eq!(part2(INPUT), 9);
    }
}

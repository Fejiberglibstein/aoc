use std::{collections::HashMap, fs, iter::Cycle, slice::Iter};

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(PartialEq, Eq)]
enum Completion {
    Finished,
    Continue,
}

pub fn part1(input: &str) -> usize {
    let mut dirs = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ]
    .iter()
    .cycle();

    let grid = input
        .lines()
        .map(|v| v.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut iter = grid.iter().enumerate().flat_map(|(y, list)| {
        list.iter()
            .enumerate()
            .map(move |(x, char)| (x as i64, y as i64, char))
    });
    let (mut x, mut y) = iter
        .find(|(_, _, &char)| char == '^')
        .map(|(x, y, _)| (x, y))
        .unwrap();

    let mut direction = dirs.next().unwrap();

    let mut set = HashMap::new();
    set.insert((x, y), ());

    let mut res = Completion::Continue;
    let mut pts: Vec<(i64, i64)>;
    while res == Completion::Continue {
        (res, pts) = move_in(&grid, *direction, x, y);

        (x, y) = *pts.last().unwrap();
        pts.iter().for_each(|&v| {
            set.insert(v, ());
        });
        direction = dirs.next().unwrap();
    }
    set.len()
}

fn print_grid(mut grid: Vec<Vec<char>>, set: HashMap<(i64, i64, Direction), ()>) {
    set.keys().for_each(|(x, y, dir)| {
        grid[*y as usize][*x as usize] = match dir {
            Direction::North => '^',
            Direction::South => 'v',
            Direction::East => '>',
            Direction::West => '<',
        }
    });

    let grid = grid
        .iter()
        .map(|v| v.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");

    println!("{grid}\n");
}

pub fn part2(input: &str) -> usize {
    let mut dirs = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ]
    .iter()
    .cycle();

    let grid = input
        .lines()
        .map(|v| v.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut iter = grid.iter().enumerate().flat_map(|(y, list)| {
        list.iter()
            .enumerate()
            .map(move |(x, char)| (x as i64, y as i64, char))
    });

    let (mut x, mut y) = iter
        .find(|(_, _, &char)| char == '^')
        .map(|(x, y, _)| (x, y))
        .unwrap();
    let (ix, iy) = (x, y);

    let mut direction = *dirs.next().unwrap();
    let mut loops = 0;

    let mut added = HashMap::new();

    let mut res = Completion::Continue;
    let mut pts: Vec<(i64, i64)>;
    while res == Completion::Continue {
        (res, pts) = move_in(&grid, direction, x, y);

        let last = *pts.last().unwrap();
        let next_dir = *dirs.next().unwrap();

        pts.iter()
            .filter(|pt| check_loop(grid.clone(), ix, iy, pt.0, pt.1))
            .for_each(|v| {
                if !added.contains_key(v) {
                    added.insert(*v, ());
                    loops += 1;
                }
            });
        direction = next_dir;
        (x, y) = last;
    }

    loops
}

fn check_loop(
    mut grid: Vec<Vec<char>>,
    mut x: i64,
    mut y: i64,
    block_x: i64,
    block_y: i64,
) -> bool {
    let mut set: HashMap<(i64, i64, Direction), ()> = HashMap::new();
    grid[block_y as usize][block_x as usize] = '#';

    let mut dirs = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ]
    .iter()
    .cycle();
    let mut direction = *dirs.next().unwrap();

    let mut res = Completion::Continue;
    let mut pts: Vec<(i64, i64)>;

    while res == Completion::Continue {
        (res, pts) = move_in(&grid, direction, x, y);
        let last = match pts.last() {
            Some(v) => *v,
            None => {
                direction = *dirs.next().unwrap();
                continue;
            }
        };
        let next_dir = *dirs.next().unwrap();

        if set.contains_key(&(last.0, last.1, next_dir)) {
            // print_grid(grid.to_vec(), set);
            return true;
        }
        set.insert((last.0, last.1, next_dir), ());

        (x, y) = last;
        direction = next_dir;
    }
    false
}

fn move_in(
    grid: &[Vec<char>],
    dir: Direction,
    mut x: i64,
    mut y: i64,
) -> (Completion, Vec<(i64, i64)>) {
    let height = grid.len();
    let width = grid[0].len();

    let mut pts = Vec::new();
    loop {
        let (mut pot_x, mut pot_y) = (x, y);
        match dir {
            Direction::North => pot_y -= 1,
            Direction::South => pot_y += 1,
            Direction::East => pot_x += 1,
            Direction::West => pot_x -= 1,
        }
        if !((0..height).contains(&(pot_y as usize)) && (0..width).contains(&(pot_x as usize))) {
            return (Completion::Finished, pts);
        }

        if grid.get(pot_y as usize).unwrap().get(pot_x as usize) == Some(&'#') {
            return (Completion::Continue, pts);
        } else {
            x = pot_x;
            y = pot_y;
            pts.push((x, y));
        }
    }
}

pub fn run() {
    let input = fs::read_to_string("./inputs/day6.txt").unwrap();
    println!("PART 1: {}", part1(&input));
    println!("PART 2: {}", part2(&input));
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test() {
        assert_eq!(part1(INPUT), 41);
        assert_eq!(part2(INPUT), 6);
    }

    #[test]
    fn test_stuff() {
        let input = "\
...........
.#.........
..........#
....#......
........#..
#^.........
...#...#...
...........
.#.......#.";
        assert_eq!(part2(input), 7)
    }

    #[test]
    fn test_stuff2() {
        let input = "\
.##..
....#
.....
.^.#.
.....
.....";
        assert_eq!(part2(input), 1)
    }
}

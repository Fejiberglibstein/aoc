use itertools::Itertools;
use std::fs;

fn parse1(input: &str) -> Vec<Option<usize>> {
    let mut state = true;
    let mut i = 0;
    input
        .chars()
        .flat_map(|ch| {
            if ch == '\n' {
                return Vec::new();
            }
            let digit = ch.to_digit(10).unwrap();
            state = !state;
            if state {
                (0..digit).map(|_| None).collect_vec()
            } else {
                let old_i = i;
                i += 1;
                (0..digit).map(|_| Some(old_i)).collect_vec()
            }
        })
        .collect_vec()
}

pub fn part1(input: &str) -> usize {
    let mut input = parse1(input);

    loop {
        let Some(pos) = input.iter().position(Option::is_none) else {
            break;
        };
        let last: usize = (|| loop {
            match input.pop() {
                Some(Some(c)) => return c,
                _ => continue,
            }
        })();

        input[pos] = Some(last);
    }
    input.iter().enumerate().map(|(i, v)| v.unwrap() * i).sum()
}

fn parse2(input: &str) -> Vec<FileData> {
    let mut state = true;
    let mut i = 0;
    input
        .chars()
        .flat_map(|ch| {
            if ch == '\n' {
                return None;
            }
            let digit = ch.to_digit(10).unwrap();
            state = !state;
            Some(if state {
                FileData {
                    size: digit,
                    num: None,
                    moved: true,
                }
            } else {
                let old_i = i;
                i += 1;
                FileData {
                    size: digit,
                    num: Some(old_i),
                    moved: old_i == 0,
                }
            })
        })
        .collect_vec()
}

#[derive(Debug, Clone)]
struct FileData {
    size: u32,
    num: Option<u32>,
    moved: bool,
}

pub fn part2(input: &str) -> usize {
    let mut input = parse2(input);

    loop {
        // println!(
        //     "{}",
        //     input
        //         .iter()
        //         .map(|v| v
        //             .num
        //             .map(|v| v.to_string())
        //             .unwrap_or(".".to_string())
        //             .repeat(v.size as usize))
        //         .join("")
        // );

        loop {
            match input.last() {
                Some(v) if v.num.is_none() => {
                    input.pop();
                }
                _ => break,
            }
        }

        let clone = input.clone();
        let Some(last) = clone
            .iter()
            .enumerate()
            .rev()
            .find(|(_, v)| {v.num.is_some() && !v.moved}) else {
                break;
            };

        let pos = match input
            .iter()
            .enumerate()
            .position(|(i, v)| v.num.is_none() && v.size >= last.1.size && i < last.0)
        {
            Some(pos) => pos,
            None => {
                input[last.0].moved = true;
                continue;
            }
        };
        input[pos].size -= last.1.size;
        input[last.0].num = None;
        input.insert(
            pos,
            FileData {
                size: last.1.size,
                num: last.1.num,
                moved: true,
            },
        );
    }

    let mut i = 0;
    input
        .iter()
        .map(|v| {
            (0..v.size)
                .map(|_| {
                    let old_i = i;
                    i += 1;
                    v.num.unwrap_or_default() as usize * old_i
                })
                .sum::<usize>()
        })
        .sum()
}

pub fn run() {
    let input = fs::read_to_string("./inputs/day9.txt").unwrap();
    // println!("PART 1: {}", part1(&input));
    println!("PART 2: {}", part2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input: &str = "2333133121414131402";

        assert_eq!(part1(input), 1928);
        assert_eq!(part2(input), 2858);
    }
}

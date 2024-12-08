use std::{cmp::Ordering, fs};

pub fn part1(input: &str) -> usize {
    let mut count = 0;
    'main: for line in input.lines() {
        let chars = line.split_whitespace();
        let mut acc = None;
        let mut last_num = None;
        for char in chars {
            let num: usize = char.parse().unwrap();
            let Some(last) = last_num else {
                last_num = Some(num);
                continue;
            };
            last_num = Some(num);

            match acc {
                None => {
                    acc = if (1..=3).contains(&last.abs_diff(num)) {
                        Some(last.partial_cmp(&num))
                    } else {
                        continue 'main;
                    }
                }
                Some(v) => {
                    if !((1..=3).contains(&last.abs_diff(num)) && last.partial_cmp(&num) == v) {
                        continue 'main;
                    }
                }
            }
        }
        count += 1;
    }
    count
}

pub fn part2(input: &str) -> usize {
    let mut count = 0;
    for line in input.lines() {
        let chars = line.split_whitespace();

        'main: for (i, _) in chars.clone().enumerate() {
            let mut acc = None;
            let mut last_num = None;
            let mut list = chars.clone().collect::<Vec<_>>();
            list.remove(i);

            for char in list {
                let num: usize = char.parse().unwrap();
                let Some(last) = last_num else {
                    last_num = Some(num);
                    continue;
                };
                last_num = Some(num);

                match acc {
                    None => {
                        acc = if (1..=3).contains(&last.abs_diff(num)) {
                            Some(last.partial_cmp(&num))
                        } else {
                            continue 'main;
                        }
                    }
                    Some(v) => {
                        if !((1..=3).contains(&last.abs_diff(num)) && last.partial_cmp(&num) == v) {
                            continue 'main;
                        }
                    }
                }
            }
            count += 1;
            break;
        }
    }
    count
}

pub fn run() {
    let input = fs::read_to_string("../inputs/day2.txt").unwrap();
    println!("PART 1: {}", part1(&input));
    println!("PART 2: {}", part2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
    #[test]
    fn test() {
        assert_eq!(part1(INPUT), 2);
        assert_eq!(part2(INPUT), 4);
    }
}

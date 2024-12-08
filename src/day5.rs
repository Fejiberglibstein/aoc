use std::{cmp::Ordering, fs};

pub fn part1(input: &str) -> usize {
    let mut split = input.split("\n\n");
    let (map, nums) = (split.next().unwrap(), split.next().unwrap());

    let map = map
        .lines()
        .map(|v| {
            let mut iter = v.split('|');
            (
                iter.next().unwrap().parse::<usize>().unwrap(),
                iter.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let nums = nums
        .lines()
        .map(|v| v.split(',').map(|n| n.parse::<usize>().unwrap()));

    let mut res = 0;
    'line: for mut line in nums {
        let line_vec = line.clone().collect::<Vec<_>>();
        let mut last = line.next().unwrap();
        for num in line {
            match map.iter().find(|&&v| v == (last, num)) {
                Some(_) => {}
                None => {
                    continue 'line;
                }
            }
            last = num;
        }
        res += line_vec.get(line_vec.len() / 2).unwrap();
    }
    res
}

fn sort(mut line: Vec<usize>, map: Vec<(usize, usize)>) -> usize {
    line.sort_by(|&a, &b| {
        if map.iter().any(|&n| n == (a, b)) {
            Ordering::Greater
        } else if map.iter().any(|&n| n == (b, a)) {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    });

    return *line.get(line.len() / 2).unwrap();
}

pub fn part2(input: &str) -> usize {
    let mut split = input.split("\n\n");
    let (map, nums) = (split.next().unwrap(), split.next().unwrap());

    let map = map
        .lines()
        .map(|v| {
            let mut iter = v.split('|');
            (
                iter.next().unwrap().parse::<usize>().unwrap(),
                iter.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let nums = nums
        .lines()
        .map(|v| v.split(',').map(|n| n.parse::<usize>().unwrap()));

    let mut res = 0;
    'line: for mut line in nums {
        let line_vec = line.clone().collect::<Vec<_>>();
        let mut last = line.next().unwrap();
        for num in line {
            match map.iter().find(|&&v| v == (last, num)) {
                Some(_) => {}
                None => {
                    res += sort(line_vec, map.clone());
                    continue 'line;
                }
            }
            last = num;
        }
    }
    res
}

pub fn run() {
    let input = fs::read_to_string("./inputs/day5.txt").unwrap();
    println!("PART 1: {}", part1(&input));
    println!("PART 2: {}", part2(&input));
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    #[test]
    fn test() {
        assert_eq!(part1(INPUT), 143);
        assert_eq!(part2(INPUT), 123);
    }
}

use itertools::Itertools;
use std::fs;

enum Operation {
    Add,
    Multiply,
    Concat,
}

impl Operation {
    pub fn do_op(&self, a: u64, b: u64) -> u64 {
        match self {
            Self::Add => a + b,
            Self::Multiply => a * b,
            Self::Concat => (a.to_string() + &b.to_string()).parse().unwrap(),
        }
    }
}

fn parse(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|v| {
            let (l, r) = v.split_once(": ").unwrap();
            let r = r
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect::<Vec<_>>();
            (l.parse().unwrap(), r)
        })
        .collect::<Vec<_>>()
}

pub fn part1(input: &str) -> usize {
    let ops = [Operation::Add, Operation::Multiply];
    let mut input = parse(input);
    input
        .iter_mut()
        .map(|(ex, nums)| {
            let first = nums.remove(0);
            (0..nums.len())
                .map(|_| ops.iter())
                .multi_cartesian_product()
                .any(|ops| {
                    *ex == nums
                        .iter()
                        .zip(ops)
                        .fold(first, |a, (num, op)| op.do_op(a, *num))
                })
                .then_some(*ex as usize)
                .unwrap_or_default()
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let ops = [Operation::Add, Operation::Multiply, Operation::Concat];
    let mut input = parse(input);
    input
        .iter_mut()
        .map(|(ex, nums)| {
            let first = nums.remove(0);
            (0..nums.len())
                .map(|_| ops.iter())
                .multi_cartesian_product()
                .any(|ops| {
                    *ex == nums
                        .iter()
                        .zip(ops)
                        .fold(first, |a, (num, op)| op.do_op(a, *num))
                })
                .then_some(*ex as usize)
                .unwrap_or_default()
        })
        .sum()
}

pub fn run() {
    let input = fs::read_to_string("./inputs/day7.txt").unwrap();
    println!("PART 1: {}", part1(&input));
    println!("PART 2: {}", part2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

        assert_eq!(part1(input), 3749);
        assert_eq!(part2(input), 11387);
    }
}

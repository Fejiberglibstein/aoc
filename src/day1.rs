use std::fs;

pub fn part1(input: &str) -> usize {
    let (mut left, mut right): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|v| {
            let mut split = v.split_whitespace();
            (
                split.next().unwrap().parse::<usize>().unwrap(),
                split.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .unzip();

    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(&a, &b)| a.abs_diff(b))
        .sum()
}

pub fn part2(input: &str) -> usize {
    let (left, right): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|v| {
            let mut split = v.split_whitespace();
            (
                split.next().unwrap().parse::<usize>().unwrap(),
                split.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .unzip();

    left.iter()
        .map(|&l| l * right.iter().filter(|&&r| r == l).count())
        .sum()
}

pub fn run() {
    let input = fs::read_to_string("../inputs/day1.txt").unwrap();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
    #[test]
    fn test() {
        assert_eq!(part1(INPUT), 11);
        assert_eq!(part2(INPUT), 31);
    }

}

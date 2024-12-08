use std::fs;

use regex::Regex;

pub fn part1(input: &str) -> usize {
    let regex = Regex::new("mul\\((\\d*),(\\d*)\\)").unwrap();

    regex
        .captures_iter(input)
        .map(|v| v.extract())
        .map(|(_, [one, two])| one.parse::<usize>().unwrap() * two.parse::<usize>().unwrap())
        .sum()
}

pub fn part2(input: &str) -> usize {
    let regex = Regex::new("mul\\((\\d*),(\\d*)\\)").unwrap();

    let input = "don't()do()".to_string() + input;
    dbg!(&input);
    let input = input
        .split("don't()")
        .map(|v| v.split_once("do()").unwrap_or_default().1)
        .collect::<String>();

    regex
        .captures_iter(&input)
        .map(|v| v.extract())
        .map(|(_, [one, two])| one.parse::<usize>().unwrap() * two.parse::<usize>().unwrap())
        .sum()
}

pub fn run() {
    let input = fs::read_to_string("./inputs/day3.txt").unwrap();
    println!("PART 1: {}", part1(&input));
    println!("PART 2: {}", part2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;

        assert_eq!(part1(input), 161);
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part2(input), 48);
    }
}

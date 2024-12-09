use aoc_2024::run_puzzle;
use regex::Regex;

const INPUT: &str = include_str!("../../input/day03.txt");

fn main() {
    run_puzzle!("Part 1", || mul(INPUT));
    run_puzzle!("Part 2", || mul_2(INPUT));
}

fn mul_2(s: &str) -> u64 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut enabled = true;
    let pairs: Vec<(u64, u64)> = re
        .captures_iter(s)
        .filter_map(|cap| {
            if cap.get(0).unwrap().as_str() == "do()" {
                enabled = true;
            } else if cap.get(0).unwrap().as_str() == "don't()" {
                enabled = false;
            }
            if enabled && cap.get(1).is_some() && cap.get(2).is_some() {
                let first = cap[1].parse().ok();
                let second = cap[2].parse().ok();
                match (first, second) {
                    (Some(f), Some(s)) => Some((f, s)),
                    _ => None,
                }
            } else {
                None
            }
        })
        .collect();

    pairs
        .iter()
        .map(|(a, b)| a * b)
        .collect::<Vec<_>>()
        .into_iter()
        .sum()
}

fn mul(s: &str) -> u64 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let pairs: Vec<(u64, u64)> = re
        .captures_iter(s)
        .filter_map(|cap| {
            let first = cap[1].parse().ok();
            let second = cap[2].parse().ok();
            match (first, second) {
                (Some(f), Some(s)) => Some((f, s)),
                _ => None,
            }
        })
        .collect();

    pairs
        .iter()
        .map(|(a, b)| a * b)
        .collect::<Vec<_>>()
        .into_iter()
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("../../tests_input/day03.txt");

    #[test]
    fn test_mul() {
        assert_eq!(mul(INPUT), 161);
    }

    #[test]
    fn test_mul_2() {
        assert_eq!(mul_2(INPUT), 48);
    }
}

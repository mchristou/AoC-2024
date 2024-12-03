use std::collections::HashMap;

use aoc_2024::run_puzzle;

const INPUT: &str = include_str!("../../input/day01.txt");

fn main() {
    run_puzzle!("Part1", || sum_distances(INPUT));
    run_puzzle!("Part2", || similarity(INPUT));
}

fn similarity(s: &str) -> u64 {
    let [l, r] = parse(s);

    let mut freq_list = HashMap::new();

    for key in r {
        *freq_list.entry(key).or_insert(0) += 1;
    }

    l.iter()
        .map(|left_key| freq_list.get(left_key).cloned().unwrap_or_default() * left_key)
        .sum()
}

fn sum_distances(s: &str) -> u64 {
    let [mut l, mut r] = parse(s);

    l.sort_unstable();
    r.sort_unstable();

    l.iter().zip(&r).map(|(l, &r)| l.abs_diff(r)).sum()
}

fn parse(s: &str) -> [Vec<u64>; 2] {
    let mut list_l = Vec::with_capacity(s.lines().count());
    let mut list_r = Vec::with_capacity(s.lines().count());

    for line in s.lines() {
        let mut parts = line.split_whitespace();

        let l = parts.next().expect("Missing left number");
        let r = parts.next().expect("Missing right number");

        list_l.push(l.parse().expect("Invalid left number"));
        list_r.push(r.parse().expect("Invalid right number"));
    }

    [list_l, list_r]
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "\
                        3 4
                        4 3
                        2 5
                        1 3
                        3 9
                        3 3";

    #[test]
    fn test_sum_distances() {
        assert_eq!(sum_distances(INPUT), 11);
    }

    #[test]
    fn test_similarity() {
        assert_eq!(similarity(INPUT), 31);
    }
}

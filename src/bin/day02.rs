use aoc_2024::run_puzzle;

const INPUT: &str = include_str!("../../input/day02.txt");

fn main() {
    run_puzzle!("Part 1", || save_report(INPUT));
}

fn save_report(s: &str) -> u64 {
    let reports = parse(s);

    reports
        .iter()
        .filter(|report| {
            let increasing = report[1] > report[0];
            report.windows(2).all(|pair| {
                let diff = (pair[1] - pair[0]).abs();
                (1..=3).contains(&diff) && (pair[1] > pair[0]) == increasing
            })
        })
        .count() as u64
}

fn parse(s: &str) -> Vec<Vec<i32>> {
    s.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|v| v.parse::<_>().expect("Invalid number in input"))
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("../../tests_input/day02.txt");

    #[test]
    fn test_safe_reports() {
        assert_eq!(save_report(INPUT), 2);
    }
}

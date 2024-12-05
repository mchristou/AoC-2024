use aoc_2024::run_puzzle;

const INPUT: &str = include_str!("../../input/day02.txt");

fn main() {
    run_puzzle!("Part 1", || save_report(INPUT));
    run_puzzle!("Part 2", || problem_dampener(INPUT));
}

fn problem_dampener(s: &str) -> u64 {
    let reports = parse(s);

    reports
        .iter()
        .filter(|report| {
            is_safe(report) || (0..report.len()).any(|i| is_safe(&remove_one(report, i)))
        })
        .count() as u64
}

fn save_report(s: &str) -> u64 {
    let reports = parse(s);

    reports.iter().filter(|report| is_safe(report)).count() as u64
}

fn is_safe(report: &[i32]) -> bool {
    let increasing = report[1] > report[0];
    report.windows(2).all(|pair| {
        let diff = (pair[1] - pair[0]).abs();
        (1..=3).contains(&diff) && (pair[1] > pair[0]) == increasing
    })
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

fn remove_one(report: &[i32], index: usize) -> Vec<i32> {
    report
        .iter()
        .enumerate()
        .filter_map(|(i, &x)| if i == index { None } else { Some(x) })
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

    #[test]
    fn test_problem_dampener() {
        assert_eq!(problem_dampener(INPUT), 4);
    }
}

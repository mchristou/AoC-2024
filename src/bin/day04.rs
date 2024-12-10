use aoc_2024::run_puzzle;

const INPUT: &str = include_str!("../../input/day04.txt");

fn main() {
    run_puzzle!("Part 1", || count_xmas(INPUT));
}

fn count_xmas(s: &str) -> u64 {
    let grid = parse(s);
    let mut count = 0;

    for row in grid.iter() {
        let row_string: String = row.iter().collect();
        count += count_matches(&row_string);

        let reversed_row_string: String = row.iter().rev().collect();
        count += count_matches(&reversed_row_string);
    }

    for col in 0..grid[0].len() {
        let column: String = grid.iter().map(|row| row[col]).collect();
        count += count_matches(&column);

        let reversed_column: String = grid.iter().rev().map(|row| row[col]).collect();
        count += count_matches(&reversed_column);
    }

    count += count_diagonals(&grid);

    count as u64
}

fn count_diagonals(grid: &[Vec<char>]) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for d in -(rows as isize - 1)..cols as isize {
        let mut diagonal = String::new();
        for (i, _item) in grid.iter().enumerate().take(rows) {
            let j = (i as isize + d) as usize;
            if j < cols {
                diagonal.push(grid[i][j]);
            }
        }

        count += count_matches(&diagonal);

        let reversed_diagonal: String = diagonal.chars().rev().collect();
        count += count_matches(&reversed_diagonal);
    }

    for d in 0..(rows + cols - 1) {
        let mut diagonal = String::new();
        for (i, _item) in grid.iter().enumerate().take(rows) {
            let j = d as isize - i as isize;
            if j >= 0 && j < cols as isize {
                diagonal.push(grid[i][j as usize]);
            }
        }
        count += count_matches(&diagonal);
        let reversed_diagonal: String = diagonal.chars().rev().collect();
        count += count_matches(&reversed_diagonal);
    }

    count
}

fn count_matches(input: &str) -> usize {
    input.matches("XMAS").count()
}

fn parse(s: &str) -> Vec<Vec<char>> {
    s.lines().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../tests_input/day04.txt");

    #[test]
    fn test_count_xmas() {
        assert_eq!(count_xmas(INPUT), 18);
    }
}

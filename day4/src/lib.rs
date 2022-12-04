use std::{fs::read, io::BufRead};

fn input_lines() -> Vec<String> {
    read("in.txt")
        .expect("can't open in.txt")
        .lines()
        .filter_map(Result::ok)
        .collect()
}

fn parse_line(line: &str) -> Vec<u32> {
    line.split(',')
        .flat_map(|e| e.split('-'))
        .filter_map(|e| e.parse::<u32>().ok())
        .collect()
}

pub fn part1() -> u32 {
    input_lines()
        .iter()
        .map(|line| match parse_line(line)[..] {
            [ax, ay, bx, by, ..] if ax <= bx && by <= ay || bx <= ax && ay <= by => 1,
            _ => 0,
        })
        .sum::<u32>()
}

pub fn part2() -> u32 {
    input_lines()
        .iter()
        .map(|line| match parse_line(line)[..] {
            [ax, ay, bx, by, ..] if ax <= bx && bx <= ay || bx <= ax && ax <= by => 1,
            _ => 0,
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        eprintln!("part1: {}", part1());
        assert_eq!(560, part1());
    }
    #[test]
    fn test2() {
        eprintln!("part2: {}", part2());
        assert_eq!(839, part2());
    }
}

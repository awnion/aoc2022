use std::{collections::HashSet, fs::read, io::BufRead};

fn input_lines() -> Vec<String> {
    read("in.txt")
        .expect("can't open in.txt")
        .lines()
        .filter_map(Result::ok)
        .collect()
}

fn fetch_input(input: Vec<String>) -> Vec<char> {
    input.first().unwrap().chars().collect()
}

fn find_marker(message: Vec<char>, n: usize) -> usize {
    message
        .windows(n)
        .enumerate()
        .find(|(_, window)| window.iter().collect::<HashSet<_>>().len() >= n)
        .unwrap()
        .0
        + n
}

pub fn part1() -> usize {
    find_marker(fetch_input(input_lines()), 4)
}

pub fn part2() -> usize {
    find_marker(fetch_input(input_lines()), 14)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        eprintln!("part1: {}", part1());
        assert_eq!(1080, part1());
    }
    #[test]
    fn test2() {
        eprintln!("part2: {}", part2());
        assert_eq!(3645, part2());
    }
}

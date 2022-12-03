use std::{fs::read, io::BufRead};

pub fn part1() -> i32 {
    read("in.txt")
        .unwrap()
        .lines()
        .map(|line| match line.unwrap().as_str() {
            "A X" => 3 + 1,
            "A Y" => 6 + 2,
            "A Z" => 0 + 3,
            "B X" => 0 + 1,
            "B Y" => 3 + 2,
            "B Z" => 6 + 3,
            "C X" => 6 + 1,
            "C Y" => 0 + 2,
            "C Z" => 3 + 3,
            _ => 0,
        })
        .sum()
}

pub fn part2() -> i32 {
    read("in.txt")
        .unwrap()
        .lines()
        .map(|line| match line.unwrap().as_str() {
            "A X" => 0 + 3,
            "A Y" => 3 + 1, //
            "A Z" => 6 + 2,
            "B X" => 0 + 1, //
            "B Y" => 3 + 2, //
            "B Z" => 6 + 3,
            "C X" => 0 + 2,
            "C Y" => 3 + 3, //
            "C Z" => 6 + 1,
            _ => 0,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        eprintln!("part1: {}", part1());
        assert_eq!(10595, part1());
    }
    #[test]
    fn test2() {
        eprintln!("part2: {}", part2());
        assert_eq!(9541, part2());
    }
}

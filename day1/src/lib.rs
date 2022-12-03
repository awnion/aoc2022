use std::{fs::read, io::BufRead};

pub fn part1() -> i32 {
    read("in.txt")
        .unwrap()
        .lines()
        .filter_map(|elf| elf.ok())
        .collect::<Vec<_>>()
        .split(|elf| elf == "")
        .map(|elf| elf.iter().filter_map(|e| e.parse::<i32>().ok()).sum())
        .max()
        .unwrap()
}

pub fn part2() -> i32 {
    let mut res = read("in.txt")
        .unwrap()
        .lines()
        .filter_map(|elf| elf.ok())
        .collect::<Vec<_>>()
        .split(|elf| elf == "")
        .map(|elf| elf.iter().filter_map(|e| e.parse::<i32>().ok()).sum())
        .collect::<Vec<i32>>();
    res.sort_unstable();
    res.iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        eprintln!("part1: {}", part1());
        assert_eq!(69528, part1());
    }
    #[test]
    fn test2() {
        eprintln!("part2: {}", part2());
        assert_eq!(206152, part2());
    }
}

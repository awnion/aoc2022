#![feature(iter_array_chunks)]
use std::{collections::HashSet, fs::read, io::BufRead};

fn input_lines() -> Vec<String> {
    read("in.txt")
        .expect("can't open in.txt")
        .lines()
        .filter_map(Result::ok)
        .collect()
}

trait Cost {
    type Output;

    fn cost(&self) -> Self::Output;
}

impl Cost for char {
    type Output = u32;

    fn cost(&self) -> Self::Output {
        match self {
            &upper @ 'A'..='Z' => upper as Self::Output - 'A' as Self::Output + 27,
            &lower @ 'a'..='z' => lower as Self::Output - 'a' as Self::Output + 1,
            _ => 0,
        }
    }
}

trait ToHashSet {
    type Item;

    fn to_hash_set(&self) -> HashSet<Self::Item>;
}

impl ToHashSet for str {
    type Item = char;

    fn to_hash_set(&self) -> HashSet<Self::Item> {
        self.chars().collect()
    }
}

pub fn part1() -> u32 {
    input_lines()
        .iter()
        .map(|line| {
            let (a, b) = line.split_at(line.len() / 2);
            a.to_hash_set()
                .intersection(&b.to_hash_set())
                .map(Cost::cost)
                .sum::<u32>()
        })
        .sum()
}

pub fn part2() -> u32 {
    input_lines()
        .iter()
        .array_chunks::<3>()
        .map(|group| {
            group
                .iter()
                .map(|line| line.as_str().to_hash_set())
                .reduce(|acc, x| acc.intersection(&x).copied().collect())
                .unwrap()
                .iter()
                .map(Cost::cost)
                .sum::<u32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        eprintln!("part1: {}", part1());
        assert_eq!(7872, part1());
    }
    #[test]
    fn test2() {
        eprintln!("part2: {}", part2());
        assert_eq!(2497, part2());
    }
}

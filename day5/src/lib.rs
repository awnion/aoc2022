use std::{fs::read, io::BufRead};

fn input_lines() -> Vec<String> {
    read("in.txt")
        .expect("can't open in.txt")
        .lines()
        .filter_map(Result::ok)
        .collect()
}

fn parse_line(line: &str) -> Option<(usize, usize, usize)> {
    match line
        .split_ascii_whitespace()
        .filter_map(|e| e.parse::<usize>().ok())
        .collect::<Vec<_>>()[..]
    {
        [a, b, c, ..] => Some((a, b, c)),
        _ => None,
    }
}

fn fetch_map(raw: Vec<String>) -> Vec<Vec<char>> {
    let mut res = Vec::new();

    raw.iter().rev().for_each(|line| {
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .for_each(|(i, ch)| {
                if i >= res.len() {
                    res.push(Vec::new())
                };
                match ch {
                    ch @ 'A'..='Z' => res[i].push(ch),
                    _ => return,
                }
            })
    });

    res
}

fn read_input() -> (Vec<Vec<char>>, Vec<String>) {
    let lines = input_lines();
    (
        fetch_map(lines.iter().take(8).cloned().collect()),
        lines.iter().skip(10).cloned().collect(),
    )
}

pub fn part1() -> String {
    let (mut map, lines) = read_input();
    lines
        .iter()
        .filter_map(|line| parse_line(line))
        .for_each(|(count, from, to)| {
            let range = map[from - 1].len().saturating_sub(count)..;
            let drained: Vec<char> = map[from - 1].drain(range).collect();
            map[to - 1].extend(drained.iter().rev());
        });
    map.iter().filter_map(|e| e.last()).collect::<String>()
}

pub fn part2() -> String {
    let (mut map, lines) = read_input();
    lines
        .iter()
        .filter_map(|line| parse_line(line))
        .for_each(|(count, from, to)| {
            let range = map[from - 1].len().saturating_sub(count)..;
            let drained: Vec<char> = map[from - 1].drain(range).collect();
            map[to - 1].extend(drained.iter());
        });
    map.iter().filter_map(|e| e.last()).collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        eprintln!("part1: {}", part1());
        assert_eq!("VQZNJMWTR", part1());
    }
    #[test]
    fn test2() {
        eprintln!("part2: {}", part2());
        assert_eq!("NLCDCLVMQ", part2());
    }
}

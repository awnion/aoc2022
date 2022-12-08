use std::{collections::HashSet, fs::read, io::BufRead};

fn input_lines() -> Vec<String> {
    read("in.txt")
        .expect("can't open in.txt")
        .lines()
        .filter_map(Result::ok)
        .collect()
}

fn tead_tree_map() -> Vec<Vec<u32>> {
    input_lines()
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|tree| tree as u32 - '0' as u32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn bump_indexes<'a>(it: impl Iterator<Item = &'a u32>) -> HashSet<usize> {
    let mut res = HashSet::new();
    let mut max = -1i32;
    it.enumerate().for_each(|(i, &v)| {
        if v as i32 > max {
            res.insert(i);
            max = v as i32;
        }
    });

    res
}

pub fn part1() -> u32 {
    let a = tead_tree_map();
    let mut s: HashSet<(usize, usize)> = HashSet::new();
    for (i, row) in a.iter().enumerate() {
        let l = row.len();
        let mut indexes = bump_indexes(row.iter());
        indexes.extend(bump_indexes(row.iter().rev()).iter().map(|j| l - j - 1));
        indexes.iter().for_each(|&j| {
            s.insert((i, j));
        });
    }

    let m = a[0].len();
    let mut b: Vec<Vec<u32>> = Vec::new();
    for i in 0..m {
        b.push(a.iter().map(|row| row[i]).collect());
    }

    for (i, row) in b.iter().enumerate() {
        let l = row.len();
        let mut indexes = bump_indexes(row.iter());
        indexes.extend(bump_indexes(row.iter().rev()).iter().map(|j| l - j - 1));
        indexes.iter().for_each(|&j| {
            s.insert((j, i));
        });
    }
    s.len() as u32
}

fn position_score(map: &Vec<Vec<u32>>, row: usize, col: usize) -> usize {
    let t = row; // top
    let b = map.len() - row - 1; // bottom
    let l = col; // left
    let r = map[0].len() - col - 1; // right

    let current = map[row][col];

    [
        (1..t).find(|&i| map[row - i][col] >= current).or(Some(t)),
        (1..b).find(|&i| map[row + i][col] >= current).or(Some(b)),
        (1..l).find(|&i| map[row][col - i] >= current).or(Some(l)),
        (1..r).find(|&i| map[row][col + i] >= current).or(Some(r)),
    ]
    .iter()
    .map(|e| e.unwrap())
    .product()
}

pub fn part2() -> u32 {
    let a = tead_tree_map();
    a.iter()
        .enumerate()
        .filter_map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, _)| position_score(&a, i, j))
                .max()
        })
        .max()
        .unwrap() as u32
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    #[test]
    fn test1() {
        eprintln!("part1: {}", part1());
        assert_eq!(1820, part1());
    }
    #[test]
    fn test2() {
        eprintln!("part2: {}", part2());
        assert_eq!(385112, part2());
    }

    #[test]
    fn test_bump_indexes() {
        let a = vec![0, 1, 0, 2, 0, 3, 0, 0, 0];
        assert_eq!(bump_indexes(a.iter()), HashSet::from([0, 1, 3, 5]));
    }
}

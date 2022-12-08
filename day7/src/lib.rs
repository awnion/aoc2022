use std::{collections::HashMap, fs::read, io::BufRead};

fn input_lines() -> Vec<String> {
    read("in.txt")
        .expect("can't open in.txt")
        .lines()
        .filter_map(Result::ok)
        .collect()
}

fn path_join(path: &mut Vec<String>, dir: &str) {
    if dir.starts_with("/") {
        path.drain(..);
        dir.split("/").skip(1)
    } else {
        dir.split("/").skip(0)
    }
    .for_each(|el| match el {
        "." | "" => {}
        ".." => {
            path.pop();
        }
        x => {
            path.push(String::from(x));
        }
    });
}

fn construct_fs() -> HashMap<Vec<String>, i32> {
    let mut current_path = Vec::new();
    let mut fs: HashMap<Vec<String>, i32> = HashMap::new();
    fs.insert(Vec::new(), 0);

    for line in input_lines().iter() {
        let split = line.split_ascii_whitespace().collect::<Vec<_>>();
        match split[..] {
            ["$", "ls", ..] => {}
            ["$", "cd", dir, ..] => {
                path_join(&mut current_path, dir);
            }
            ["dir", name, ..] => {
                let mut path = current_path.clone();
                path.push(name.to_owned());
                fs.insert(path, 0);
            }
            [size, name, ..] => {
                let mut path = current_path.clone();
                path.push(name.to_owned());
                fs.insert(path, size.parse::<i32>().unwrap());
            }
            _ => (),
        }
    }
    let mut keys = fs.keys().cloned().collect::<Vec<_>>();
    keys.sort_by(|a, b| b.len().cmp(&a.len()));
    for key in keys.iter() {
        let &value = fs.get(key).unwrap();
        if value > 0 {
            let mut path = key.clone();
            while let Some(_) = path.pop() {
                fs.get_mut(&path).map(|x| *x -= value);
            }
        }
    }
    fs
}

pub fn part1() -> i32 {
    let fs = construct_fs();
    let mut res = 0;
    for &val in fs.values() {
        if val <= 0 && val >= -100_000 {
            res -= val;
        }
    }
    res
}
pub fn part2() -> i32 {
    let fs = construct_fs();
    let need_at_least = 30_000_000 - 70_000_000 - fs.get(&vec![]).unwrap();
    -fs.values()
        .filter(|&&val| val <= -need_at_least)
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    #[test]
    fn test1() {
        eprintln!("part1: {}", part1());
        assert_eq!(1449447, part1());
    }
    #[test]
    fn test2() {
        eprintln!("part2: {}", part2());
        assert_eq!(8679207, part2());
    }

    #[test]
    fn test_path_join() {
        let mut path: Vec<String> = Vec::new();
        path_join(&mut path, "/");
        assert_eq!(path.len(), 0);
        path_join(&mut path, "/path");
        assert_eq!(path, vec!["path"]);
        path_join(&mut path, "../../..");
        assert_eq!(path.len(), 0);
    }
}

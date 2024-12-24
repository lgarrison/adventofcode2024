#![allow(non_snake_case)]

use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

fn numeric_part(code: &str) -> usize {
    code[..code.len() - 1].parse().unwrap()
}

// adapted from https://www.reddit.com/r/adventofcode/comments/1hj2odw/comment/m3482ai/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
fn path(p: &Vec<&str>, f: &str, t: &str) -> String {
    let (fx, fy) = p
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.chars().enumerate().map(move |(x, c)| (x, y, c)))
        .filter(|(_, _, c)| *c == f.chars().next().unwrap())
        .next()
        .map(|(x, y, _)| (x, y))
        .unwrap();

    let (tx, ty) = p
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.chars().enumerate().map(move |(x, c)| (x, y, c)))
        .filter(|(_, _, c)| *c == t.chars().next().unwrap())
        .next()
        .map(|(x, y, _)| (x, y))
        .unwrap();

    fn g(p: &Vec<&str>, tx: usize, ty: usize, x: usize, y: usize, s: String) -> Vec<String> {
        let mut paths = Vec::new();
        if x == tx && y == ty {
            paths.push(s.clone() + "A");
        }
        if tx < x && p[y].chars().nth(x - 1).unwrap() != ' ' {
            let mut new_s = s.clone();
            new_s.push('<');
            paths.extend(g(p, tx, ty, x - 1, y, new_s));
        }
        if ty < y && p[y - 1].chars().nth(x).unwrap() != ' ' {
            let mut new_s = s.clone();
            new_s.push('^');
            paths.extend(g(p, tx, ty, x, y - 1, new_s));
        }
        if ty > y && p[y + 1].chars().nth(x).unwrap() != ' ' {
            let mut new_s = s.clone();
            new_s.push('v');
            paths.extend(g(p, tx, ty, x, y + 1, new_s));
        }
        if tx > x && p[y].chars().nth(x + 1).unwrap() != ' ' {
            let mut new_s = s.clone();
            new_s.push('>');
            paths.extend(g(p, tx, ty, x + 1, y, new_s));
        }
        paths
    }

    g(p, tx, ty, fx, fy, String::new())
        .into_iter()
        .min_by_key(|p| {
            p.chars()
                .zip(p.chars().skip(1))
                .filter(|(a, b)| a != b)
                .count()
        })
        .unwrap()
}

fn solve(s: &str, l: usize, cache: &mut HashMap<(String, usize), usize>) -> usize {
    if l > 25 {
        return s.len();
    }
    let key = (s.to_string(), l);
    if let Some(&val) = cache.get(&key) {
        return val;
    }
    let n = vec!["789", "456", "123", " 0A"];
    let d = vec![" ^A", "<v>"];
    let paths = (0..s.len())
        .map(|i| {
            let t = s.chars().nth(i).unwrap();
            let f = if i == 0 { 'A' } else { s.chars().nth(i - 1).unwrap() };
            let d = if l > 0 { &d } else { &n };
            path(&d, &f.to_string(), &t.to_string())
        })
        .collect::<Vec<String>>();
    let result = paths.iter().map(|path_str| solve(path_str, l + 1, cache)).sum();
    cache.insert(key, result);
    result
}

fn part2(txt: &str) -> usize {
    let codes: Vec<&str> = txt.lines().collect();

    let mut cache = HashMap::new();

    codes
        .iter()
        // .take(1)
        // .map(|c| shortest_sequence_len(c, 3 + 23)
        .map(|c| solve(c, 0, &mut cache) * numeric_part(c))
        // .inspect(|x| println!("len: {}", x))
        .sum()
}

fn main() {
    let day_x = env!("CARGO_BIN_NAME");
    let root = env!("CARGO_MANIFEST_DIR");

    let path = String::from(root) + "/src/" + day_x + "/input.txt";
    // let path = String::from(root) + "/src/" + day_x + "/test1.txt";
    let txt = fs::read_to_string(path).unwrap();

    println!("This is {}", day_x);
    // println!("Part 1: {:?}", part1(&txt));
    println!("Part 2: {:?}", part2(&txt));
}

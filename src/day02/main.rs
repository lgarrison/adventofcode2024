#![feature(iter_map_windows)]

use std::fs;

fn safe(r: &Vec<i64>) -> bool {
    let diffs: Vec<i64> = r.iter().map_windows(|[&a, &b]| b - a).collect();

    diffs.iter().all(|&x| x >= 1 && x <= 3) || diffs.iter().all(|&x| x <= -1 && x >= -3)
}

fn part1(txt: &str) -> i64 {
    txt.lines()
        .map(|l| {
            let report: Vec<i64> = l
                .split_whitespace()
                .map(|w| w.parse::<i64>().unwrap())
                .collect();

            safe(&report) as i64
        })
        .sum()
}

fn part2(txt: &str) -> i64 {
    txt.lines()
        .map(|l| {
            let report: Vec<i64> = l
                .split_whitespace()
                .map(|w| w.parse::<i64>().unwrap())
                .collect();

            (safe(&report)
                || (0..report.len()).any(|i| {
                    safe(
                        &report
                            .iter()
                            .enumerate()
                            .filter_map(|(idx, &v)| if idx != i { Some(v) } else { None })
                            .collect(),
                    )
                })) as i64
        })
        .sum()
}

fn main() {
    let day_x = env!("CARGO_BIN_NAME");
    let root = env!("CARGO_MANIFEST_DIR");

    let path = String::from(root) + "/src/" + day_x + "/input.txt";
    // let path = String::from(root) + "/src/" + day_x + "/test1.txt";
    let txt = fs::read_to_string(path).unwrap();

    println!("This is {}", day_x);
    println!("Part 1: {:?}", part1(&txt));
    println!("Part 2: {:?}", part2(&txt));
}

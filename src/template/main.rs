#![allow(non_snake_case)]

use std::fs;

fn part1(txt: &str) -> i64 {
    0
}

fn part2(txt: &str) -> i64 {
    0
}

fn main() {
    let day_x = env!("CARGO_BIN_NAME");
    let root = env!("CARGO_MANIFEST_DIR");

    //let path = String::from(root) + "/src/" + day_x + "/input.txt";
    let path = String::from(root) + "/src/" + day_x + "/test1.txt";
    let txt = fs::read_to_string(path).unwrap();

    println!("This is {}", day_x);
    println!("Part 1: {:?}", part1(&txt));
    println!("Part 2: {:?}", part2(&txt));
}

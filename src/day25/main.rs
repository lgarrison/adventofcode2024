#![allow(non_snake_case)]

use std::fs;

type Plan = [isize; 5];

fn get_keys_locks(txt: &str) -> [Vec<Plan>; 2] {
    let plans = txt.split("\n\n").collect::<Vec<&str>>();

    let mut keys: Vec<Plan> = vec![];
    let mut locks: Vec<Plan> = vec![];

    for plan in plans {
        let is_key = plan.lines().next().unwrap().chars().all(|c| c == '#');

        let mut depths: Plan = [-1; 5];

        for line in plan.lines() {
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    depths[i] += 1;
                }
            }
        }

        if is_key {
            keys.push(depths);
        } else {
            locks.push(depths);
        }
    }

    [keys, locks]
}

fn part1(txt: &str) -> usize {
    let [keys, locks] = get_keys_locks(txt);

    println!("{:?}", keys);
    println!("{:?}", locks);

    locks.iter().map(|lock| {
        keys.iter().filter(|key| {
            lock.iter().zip(key.iter()).all(|(l, k)| l + k <= 5)
        })
    }).flatten().count() as usize

}

fn part2(txt: &str) -> i64 {
    // merry christmas!
    0
}

fn main() {
    let day_x = env!("CARGO_BIN_NAME");
    let root = env!("CARGO_MANIFEST_DIR");

    let path = String::from(root) + "/src/" + day_x + "/input.txt";
    // let path = String::from(root) + "/src/" + day_x + "/test1.txt";
    let txt = fs::read_to_string(path).unwrap();

    println!("This is {}", day_x);
    println!("Part 1: {:?}", part1(&txt));
    // println!("Part 2: {:?}", part2(&txt));
}

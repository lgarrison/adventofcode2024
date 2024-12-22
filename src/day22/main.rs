#![allow(non_snake_case)]

use rayon::prelude::*;
use std::fs;

fn hash(mut state: usize, rounds: isize) -> usize {
    for _ in 0..rounds {
        state = ((state << 6) ^ state) & 0xffffff;
        state = ((state >> 5) ^ state) & 0xffffff;
        state = ((state << 11) ^ state) & 0xffffff;
    }
    state
}

fn buy_price(mut state: usize, rounds: isize, seq: [isize; 4]) -> usize {
    let mut prev_price = state % 10;

    let mut last4changes = [0; 4];
    for i in 0..rounds {
        state = hash(state, 1);
        let curr_price = state % 10;
        last4changes[3] = curr_price as isize - prev_price as isize;
        if last4changes == seq && i >= 3 {
                return curr_price;
        }
        prev_price = curr_price;
        last4changes[0] = last4changes[1];
        last4changes[1] = last4changes[2];
        last4changes[2] = last4changes[3];
    }
    0
}

fn part1(txt: &str) -> usize {
    txt.lines().map(|l| hash(l.parse().unwrap(), 2000)).sum()
}

fn part2(txt: &str) -> usize {
    let secrets: Vec<usize> = txt.lines().map(|l| l.parse().unwrap()).collect();

    // all combinations of 4 numbers where each is -9..=9
    let seqs = (-9..=9).flat_map(|a| {
        (-9..=9).flat_map(move |b| (-9..=9).flat_map(move |c| (-9..=9).map(move |d| [a, b, c, d])))
    });

    // the inverse method where you record change sequences and find
    // the optimal one is faster, but rayon is fun!
    seqs
        .par_bridge()
        .map(|seq| {
            secrets
                .iter()
                .map(|&secret| buy_price(secret, 2000, seq))
                // .inspect(|&price| println!("{:?}", price))
                .sum()
        })
        .max()
        .unwrap()
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

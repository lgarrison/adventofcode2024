use std::{collections::HashMap, fs};

fn count(stone: i64, rounds_left: i64, memo: &mut HashMap<[i64;2],i64>) -> i64 {
    // println!("stone: {}, rounds_left: {}", stone, rounds_left);
    if rounds_left == 0 {
        return 1;
    }

    if let Some(&result) = memo.get(&[stone, rounds_left]) {
        return result;
    }

    let res = if stone == 0 {
        count(1, rounds_left - 1, memo)
    } else {
        let digits = stone.ilog(10) as i64 + 1;

        if digits % 2 == 0 {
            let base = 10i64.pow((digits / 2) as u32);
            let lhs = stone / base;
            let rhs = stone % base;

            count(lhs, rounds_left - 1, memo) + count(rhs, rounds_left - 1, memo)
        } else {
            count(stone * 2024, rounds_left - 1, memo)
        }
        
    };

    memo.insert([stone, rounds_left], res);
    res
}

fn part1(txt: &str) -> i64 {
    txt.split_ascii_whitespace()
       .map(|w| count(w.parse::<i64>().unwrap(),
                            75,
                            &mut HashMap::new())
        ).sum()
}

fn part2(txt: &str) -> i64 {
    0
}

fn main() {
    let day_x = env!("CARGO_BIN_NAME");
    let root = env!("CARGO_MANIFEST_DIR");

    let path = String::from(root) + "/src/" + day_x + "/input.txt";
    // let path = String::from(root) + "/src/" + day_x + "/test1.txt";
    // let path = String::from(root) + "/src/" + day_x + "/test2.txt";
    let txt = fs::read_to_string(path).unwrap();

    println!("This is {}", day_x);
    println!("Part 1: {:?}", part1(&txt));
    println!("Part 2: {:?}", part2(&txt));
}

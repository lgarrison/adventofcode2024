use regex::Regex;
use std::fs;

fn part1(txt: &str) -> i64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let caps = re.captures_iter(txt);
    caps.map(|c| c[1].parse::<i64>().unwrap() * c[2].parse::<i64>().unwrap())
        .sum()
}

fn part2(txt: &str) -> i64 {
    let re = Regex::new(r"(mul)\((\d+),(\d+)\)|(do|don't)\(\)").unwrap();
    let caps = re.captures_iter(txt);
    let mut doit = true;
    caps.filter_map(|c| {
        if c.get(1).is_some() && doit {
            Some(c[2].parse::<i64>().unwrap() * c[3].parse::<i64>().unwrap())
        } else {
            let op = c.get(4);
            if op.is_some_and(|v| v.as_str() == "do") {
                doit = true;
            } else if op.is_some_and(|v| v.as_str() == "don't") {
                doit = false;
            }
            None
        }
    })
    .sum()
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

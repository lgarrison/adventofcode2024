use std::{collections::HashMap, fs};

fn parse(txt: &str) -> [Vec<&str>; 2] {
    let mut parts = txt.split("\n\n");
    let towels = parts.next().unwrap().split(", ").collect();
    let patterns = parts.next().unwrap().lines().collect();
    [towels, patterns]
}

type Memo<'a> = HashMap<&'a str, isize>;

fn search<'a>(pattern: &'a str, towels: &Vec<&'a str>, memo: &mut Memo<'a>) -> isize {
    if let Some(&i) = memo.get(pattern) {
        return i;
    }
    if pattern.len() == 0 {
        return 1;
    }

    let total = towels.into_iter().filter(|&&t| pattern.starts_with(t)).map(|&t| {
        // println!("{}: {}", t, pattern);
        search(&pattern[t.len()..], towels, memo)
    }).sum();
    memo.insert(pattern, total);
    total
}

fn part1(txt: &str) -> usize {
    let [towels, patterns] = parse(txt);

    patterns.into_iter().filter(|&p| search(p, &towels, &mut Memo::new()) > 0).count()
}

fn part2(txt: &str) -> isize {
    let [towels, patterns] = parse(txt);

    patterns.into_iter().map(|p| {
        search(p, &towels, &mut Memo::new())}
    ).sum()
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

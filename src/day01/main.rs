use std::collections::HashMap;
use std::fs;

fn get_vecs(txt: &str) -> (Vec<u64>, Vec<u64>) {
    let (mut vec1, mut vec2): (Vec<_>, Vec<_>) = txt
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace().map(|x| x.parse::<u64>().unwrap());
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .unzip();
    vec1.sort();
    vec2.sort();
    (vec1, vec2)
}

fn part1(txt: &str) -> u64 {
    let (vec1, vec2): (Vec<_>, Vec<_>) = get_vecs(txt);

    vec1.iter()
        .zip(vec2.iter())
        .map(|(&x, &y)| x.abs_diff(y))
        .sum()
}

fn part2(txt: &str) -> u64 {
    let (vec1, vec2) = get_vecs(txt);
    let mut counter: HashMap<u64, u64> = HashMap::new();
    for &y in vec2.iter() {
        *counter.entry(y).or_insert(0) += 1;
    }
    vec1.iter().map(|x| x * counter.get(x).unwrap_or(&0)).sum()
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

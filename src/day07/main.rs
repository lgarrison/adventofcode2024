use std::fs;

fn try_ops(res: i64, first: i64, vals: &[i64]) -> bool {
    // println!("res: {}, first: {}, vals: {:?}", res, first, vals);
    if first > res {
        return false;
    }
    if vals.len() == 0 {
        return res == first;
    } else if res < 0 {
        return false;
    }

    for op in ['*', '+', '|'] {
        let p = match op {
            '*' => first * vals[0],
            '+' => first + vals[0],
            '|' => first * 10i64.pow(vals[0].ilog10() + 1) + vals[0],
            _ => panic!("bad op"),
        };

        if try_ops(res, p, &vals[1..]) {
            return true;
        }
    }
    return false;
}

fn part1(txt: &str) -> i64 {
    txt.lines()
        .map(|line| {
            let mut nums = line
                .split_ascii_whitespace()
                .map(|w| w.trim_end_matches(':').parse::<i64>().unwrap());
            let res = nums.next().unwrap();
            let vals: Vec<i64> = nums.collect();

            if try_ops(res, vals[0], &vals[1..]) {
                res
            } else {
                0
            }
        })
        .sum()
}

fn part2(txt: &str) -> i64 {
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
    println!("Part 2: {:?}", part2(&txt));
}

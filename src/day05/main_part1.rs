use std::{collections::HashMap, fs};

type Rule = [isize; 2];

#[derive(Debug)]
struct Update {
    map: HashMap<isize, isize>,
    mid: isize,
}

fn parse_rules(txt: &str) -> Vec<Rule> {
    txt.lines().map(|l| {
        let mut r = l.split("|").map(|c| c.parse::<isize>().unwrap());
        [r.next().unwrap(), r.next().unwrap()]
    }).collect()
}

// fn argsort(data: &Vec<isize>) -> Vec<isize> {
//     let mut indices: Vec<isize> = (0..data.len() as isize).collect();
//     indices.sort_by_key(|&i| &data[i as usize]);
//     indices
// }

impl Update {
    fn from_str(txt: &str) -> Vec<Update> {
        txt.lines().map(|l| {
            let v: Vec<isize> = l.split(",").map(|c| c.parse::<isize>().unwrap()).collect();
            Update {
                map: HashMap::from_iter(v.iter().enumerate().map(|(i,x)| (*x, i as isize))),
                mid: *v.get(v.len()/2).unwrap()
            }
        }).collect()
    }

    fn check_rules(&self, rules: &Vec<Rule>) -> bool {
        rules.iter().all(|rule|
            self.map.get(&rule[0]).and_then(|i| 
                self.map.get(&rule[1]).and_then(|j| Some(i < j)).or(Some(true))
            ).or(Some(true)).unwrap()
        )
    }
}

fn parse(txt: &str) -> (Vec<Rule>, Vec<Update>) {
    let mut t = txt.split("\n\n");
    (parse_rules(t.next().unwrap()), Update::from_str(t.next().unwrap()))
}

fn part1(txt: &str) -> isize {
    let (rules, updates) = parse(txt);
    updates.iter()
        .filter_map(|update| if update.check_rules(&rules) { Some(update.mid) } else { None } )
        .sum()
}

fn part2(txt: &str) -> isize {
    let (rules, updates) = parse(txt);
}

fn main() {
    let day_x = env!("CARGO_BIN_NAME");
    let root = env!("CARGO_MANIFEST_DIR");

    // let path = String::from(root) + "/src/" + day_x + "/input.txt";
    let path = String::from(root) + "/src/" + day_x + "/test1.txt";
    let txt = fs::read_to_string(path).unwrap();

    println!("This is {}", day_x);
    println!("Part 1: {:?}", part1(&txt));
    println!("Part 2: {:?}", part2(&txt));
}

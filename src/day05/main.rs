use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs,
};

type Rule = [isize; 2];
type Update = Vec<isize>;
type RulesMap = HashMap<isize, HashSet<isize>>;

#[derive(Debug)]
struct UpdateP1 {
    map: HashMap<isize, isize>,
    vec: Vec<isize>,
    mid: isize,
}

impl UpdateP1 {
    fn from_str(txt: &str) -> Vec<UpdateP1> {
        txt.lines()
            .map(|l| {
                let v: Vec<isize> = l.split(",").map(|c| c.parse::<isize>().unwrap()).collect();
                UpdateP1 {
                    map: HashMap::from_iter(v.iter().enumerate().map(|(i, x)| (*x, i as isize))),
                    mid: *v.get(v.len() / 2).unwrap(),
                    vec: v,
                }
            })
            .collect()
    }

    fn check_rules(&self, rules: &Vec<Rule>) -> bool {
        rules.iter().all(|rule| {
            self.map
                .get(&rule[0])
                .and_then(|i| {
                    self.map
                        .get(&rule[1])
                        .and_then(|j| Some(i < j))
                        .or(Some(true))
                })
                .or(Some(true))
                .unwrap()
        })
    }
}

fn parse_rules(txt: &str) -> Vec<Rule> {
    txt.lines()
        .map(|l| {
            let mut r = l.split("|").map(|c| c.parse::<isize>().unwrap());
            [r.next().unwrap(), r.next().unwrap()]
        })
        .collect()
}

fn parse_p1(txt: &str) -> (Vec<Rule>, Vec<UpdateP1>) {
    let mut t = txt.split("\n\n");
    (
        parse_rules(t.next().unwrap()),
        UpdateP1::from_str(t.next().unwrap()),
    )
}

fn sort_by_rules(update: &Update, rules: &RulesMap) -> Update {
    let mut new = update.clone();
    new.sort_by(|a, b| {
        rules
            .get(a)
            .and_then(|r| r.contains(b).then_some(Ordering::Less))
            .unwrap_or(Ordering::Equal)
    });
    new
}

fn get_rules_map(rules: &Vec<Rule>) -> RulesMap {
    let mut rules_map = RulesMap::new();
    for rule in rules {
        rules_map
            .entry(rule[0])
            .or_insert(HashSet::new())
            .insert(rule[1]);
    }
    rules_map
}

fn part1(txt: &str) -> isize {
    let (rules, updates) = parse_p1(txt);
    updates
        .iter()
        .filter_map(|update| {
            if update.check_rules(&rules) {
                Some(update.mid)
            } else {
                None
            }
        })
        .sum()
}

fn part2(txt: &str) -> isize {
    let (rules, updates_p1) = parse_p1(txt);
    let unordered: Vec<Update> = updates_p1
        .iter()
        .filter_map(|updatep1| {
            if !updatep1.check_rules(&rules) {
                Some(updatep1.vec.clone())
            } else {
                None
            }
        })
        .collect();
    // println!("{:?}", unordered);
    let rules_map: RulesMap = get_rules_map(&rules);
    // println!("{:?}", rules_map);
    let ordered: Vec<Update> = unordered
        .iter()
        .map(|update| sort_by_rules(&update, &rules_map))
        .collect();
    // println!("{:?}", ordered);
    ordered.iter().map(|update| update[update.len() / 2]).sum()
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

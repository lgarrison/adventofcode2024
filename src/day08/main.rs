use std::{collections::{HashMap, HashSet}, fs, ops::{Add, Sub, Mul}};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Debug)]
struct Point {
    i: isize,
    j: isize,
}

#[derive(Debug)]
struct Antennas(HashMap<char, Vec<Point>>);

#[derive(Debug)]
struct Antinodes(HashSet<Point>);

const N: isize = 50;

impl Antennas {
    fn from_str(txt: &str) -> Self {
        let mut ant = HashMap::new();
        txt.lines().enumerate().for_each(|(i, line)|
            line.chars().enumerate().for_each(|(j, char)|
                if char != '.' {
                    ant.entry(char).or_insert_with(Vec::new).push(Point{i: i as isize, j: j as isize})
                }
            )
        );
        Antennas(ant)
    }
}

impl Antinodes {
    fn new() -> Self {
        Antinodes(HashSet::new())
    }
    fn set(&mut self, point: Point) {
        if point.in_bounds() {
            self.0.insert(point);
        }
    }
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            i: self.i + other.i,
            j: self.j + other.j,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            i: self.i - other.i,
            j: self.j - other.j,
        }
    }
}

impl Mul<isize> for Point {
    type Output = Self;

    fn mul(self, other: isize) -> Self {
        Self {
            i: self.i * other,
            j: self.j * other,
        }
    }
}

impl Point {
    fn antinodes(self, other: Point) -> [Point;2] {
        [
            self + (self - other),
            other + (other - self),
        ]
    }

    fn antinodes_p2(self, other: Point) -> Vec<Point> {
        let delta = other - self;
        let mut v: Vec<Point> = vec![];
        
        for k in 0..N {
            let p = self + delta * k;
            if p.in_bounds() {
                v.push(p);
            } else {
                break;
            }
        }

        for k in 0..N {
            let p = self - delta * k;
            if p.in_bounds() {
                v.push(p);
            } else {
                break;
            }
        }
        v
    }

    fn in_bounds(self) -> bool {
        self.i >= 0 && self.i < N && self.j >= 0 && self.j < N
    }
}

fn part1(txt: &str) -> usize {
    let antennas = Antennas::from_str(txt);

    let mut antinodes = Antinodes::new();
    for (_c, v) in antennas.0.iter() {
        for (i, a) in v.iter().enumerate() {
            for b in v[i+1..].iter() {
                for x in a.antinodes(*b) {
                    antinodes.set(x);
                }
            }
        }
    }

    antinodes.len()
}

fn part2(txt: &str) -> usize {
    let antennas = Antennas::from_str(txt);

    let mut antinodes = Antinodes::new();
    for (_c, v) in antennas.0.iter() {
        for (i, a) in v.iter().enumerate() {
            for b in v[i+1..].iter() {
                for x in a.antinodes_p2(*b) {
                    antinodes.set(x);
                }
            }
        }
    }

    antinodes.len()
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

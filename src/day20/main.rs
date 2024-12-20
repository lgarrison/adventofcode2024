#![allow(non_snake_case)]

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::{collections::HashMap, fs};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Map {
    map: HashMap<Point, Tile>,
    cost_map: HashMap<Point, isize>,
    start: Point,
    N: Point,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Tile {
    Empty,
    Wall,
    Start,
    End,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
struct Point {
    i: isize,
    j: isize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl std::ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            i: self.i + other.i,
            j: self.j + other.j,
        }
    }
}

impl std::ops::Add<Dir> for Point {
    type Output = Point;

    fn add(self, other: Dir) -> Point {
        match other {
            Dir::Left => Point {
                i: self.i,
                j: self.j - 1,
            },
            Dir::Right => Point {
                i: self.i,
                j: self.j + 1,
            },
            Dir::Up => Point {
                i: self.i - 1,
                j: self.j,
            },
            Dir::Down => Point {
                i: self.i + 1,
                j: self.j,
            },
        }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.i, self.j)
    }
}

impl Map {
    fn from_str(txt: &str) -> Self {
        let mut map = HashMap::new();
        let mut start = Point { i: 0, j: 0 };
        let mut N = Point { i: 0, j: 0 };
        for (i, line) in txt.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                let point = Point {
                    i: i as isize,
                    j: j as isize,
                };
                let tile: Tile = match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Wall,
                    'S' => {
                        start = point;
                        Tile::Start
                    }
                    'E' => Tile::End,
                    _ => panic!("Invalid tile: {}", c),
                };
                map.insert(point, tile);
                N.i = N.i.max(i as isize + 1);
                N.j = N.j.max(j as isize + 1);
            }
        }
        let mut ret = Self {
            map,
            start,
            N,
            cost_map: HashMap::new(),
        };
        ret.fill_cost_map();
        ret
    }

    fn fill_cost_map(&mut self) {
        let positions: Vec<Point> = self
            .map
            .iter()
            .filter(|(_, &tile)| tile != Tile::Wall)
            .map(|(&pos, _)| pos)
            .collect();
        for pos in positions {
            self.cost_map.insert(pos, self.cost_to_end(pos));
        }
    }

    fn get(&self, pos: Point) -> Tile {
        *self.map.get(&pos).unwrap()
    }

    fn is_edge(&self, pos: Point) -> bool {
        pos.i == 0 || pos.j == 0 || pos.i == self.N.i - 1 || pos.j == self.N.j - 1
    }

    fn cost_to_end(&self, pos: Point) -> isize {
        let mut heap = BinaryHeap::new();
        heap.push(Reverse((0, pos)));

        let mut visited = HashMap::new();
        visited.insert(pos, 0);

        while let Some(Reverse((cost, pos))) = heap.pop() {
            if self.get(pos) == Tile::End {
                return cost;
            }

            for (new_dir, new_cost) in self.moves(pos, Some(0)) {
                let new_pos = pos + new_dir;
                let new_cost = cost + new_cost;
                if let Some(&best_cost) = visited.get(&new_pos) {
                    if new_cost > best_cost {
                        continue;
                    }
                }
                visited.insert(new_pos, new_cost);
                heap.push(Reverse((new_cost, new_pos)));
            }
        }

        panic!("No path to end");
    }

    fn moves(&self, pos: Point, cheat: Option<isize>) -> Vec<(Dir, isize)> {
        let mut moves = vec![];
        for &dir in [Dir::Left, Dir::Right, Dir::Up, Dir::Down].iter() {
            let new_pos = pos + dir;
            if self.get(new_pos) != Tile::Wall {
                moves.push((dir, 1));
            }
        }
        moves
    }

    fn count_cheats(&self, min_save: isize, max_jump: isize) -> isize {
        // For every empty square, calculate if jumping up to max_jump
        // squares saves more than min_save distance.

        let mut count = 0;
        for (&pos, &cost) in self.cost_map.iter() {
            for ijump in -max_jump..=max_jump {
                for jjump in -max_jump..=max_jump {
                    let jump_dist = ijump.abs() + jjump.abs();
                    if jump_dist == 0 || jump_dist > max_jump {
                        continue;
                    }
                    let jump = Point { i: ijump, j: jjump };
                    let new_pos = pos + jump;
                    if let Some(new_cost) = self.cost_map.get(&new_pos) {
                        if cost - (new_cost + jump_dist) >= min_save {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }

    fn print_map(&self, path: &HashSet<Point>) {
        for i in 0..=self.N.i {
            for j in 0..=self.N.j {
                let point = Point { i, j };
                let c = match self.get(point) {
                    Tile::Empty => '.',
                    Tile::Wall => '#',
                    Tile::Start => 'S',
                    Tile::End => 'E',
                };
                if path.contains(&point) {
                    print!("O");
                } else {
                    print!("{}", c);
                }
            }
            println!();
        }
    }
}

fn part1(txt: &str) -> isize {
    let map = Map::from_str(txt);

    let orig_cost = map.cost_to_end(map.start);
    println!("orig_cost: {}", orig_cost);

    map.count_cheats(100, 2)
}

fn part2(txt: &str) -> isize {
    let map = Map::from_str(txt);

    let orig_cost = map.cost_to_end(map.start);
    println!("orig_cost: {}", orig_cost);

    map.count_cheats(100, 20)
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

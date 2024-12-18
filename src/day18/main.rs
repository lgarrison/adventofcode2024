use std::{
    collections::{HashMap, VecDeque},
    fs,
};

#[derive(Debug)]
struct Map {
    grid: HashMap<Point, Tile>,
    N: isize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Tile {
    Empty,
    Corrupted,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    i: isize,
    j: isize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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

impl Map {
    fn from_points(points: &Vec<Point>, N: isize) -> Self {
        let mut grid = HashMap::new();
        for point in points {
            grid.insert(*point, Tile::Corrupted);
        }
        Map { grid, N }
    }

    fn get(&self, p: Point) -> Option<Tile> {
        if p.i < 0 || p.i >= self.N || p.j < 0 || p.j >= self.N {
            None
        } else {
            self.grid.get(&p).or(Some(&Tile::Empty)).copied()
        }
    }

    fn moves(&self, p: Point) -> Vec<Point> {
        let mut moves = vec![];
        for &dir in [Dir::Left, Dir::Right, Dir::Up, Dir::Down].iter() {
            let new_p = p + dir;
            if self.get(new_p) == Some(Tile::Empty) {
                moves.push(new_p);
            }
        }
        moves
    }

    fn shortest_dist(&self, start: Point, end: Point) -> Option<isize> {
        let mut dists: HashMap<Point, isize> = HashMap::new();
        dists.insert(start, 0);

        let mut queue: VecDeque<Point> = VecDeque::new();
        queue.push_back(start);

        while let Some(point) = queue.pop_front() {
            let dist = dists[&point];

            if point == end {
                return Some(dist);
            }

            for next in self.moves(point) {
                if dists.get(&next).is_some_and(|&nd| nd <= dist + 1) {
                    continue;
                }
                queue.push_back(next);
                dists.insert(next, dist + 1);
            }
        }

        None
    }
}

fn get_points(txt: &str) -> Vec<Point> {
    txt.lines()
        .map(|line| {
            let mut parts = line.split(",");
            let i = parts.next().unwrap().parse().unwrap();
            let j = parts.next().unwrap().parse().unwrap();
            Point { i, j }
        })
        .collect()
}

fn part1(txt: &str) -> isize {
    let bytes = get_points(txt);
    let map = Map::from_points(&bytes.into_iter().take(1024).collect(), 71);
    let start = Point { i: 0, j: 0 };
    let end = Point {
        i: map.N - 1,
        j: map.N - 1,
    };
    map.shortest_dist(start, end).unwrap()
}

fn part2(txt: &str) -> String {
    let bytes = get_points(txt);
    let mut map = Map::from_points(&bytes.clone().into_iter().take(1024).collect(), 71);
    let start = Point { i: 0, j: 0 };
    let end = Point {
        i: map.N - 1,
        j: map.N - 1,
    };

    for point in bytes.into_iter().skip(1024) {
        map.grid.insert(point, Tile::Corrupted);

        if map.shortest_dist(start, end).is_none() {
            return format!("{},{}", point.i, point.j);
        }
    }
    panic!("No solution found");
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

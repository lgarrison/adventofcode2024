use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::{collections::HashMap, fs};

struct Map {
    map: HashMap<Point, Tile>,
    start: Point,
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

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.i, self.j)
    }
}

impl Dir {
    fn as_point(&self) -> Point {
        match self {
            Dir::Left => Point { i: 0, j: -1 },
            Dir::Right => Point { i: 0, j: 1 },
            Dir::Up => Point { i: -1, j: 0 },
            Dir::Down => Point { i: 1, j: 0 },
        }
    }

    fn left90(&self) -> Dir {
        match self {
            Dir::Left => Dir::Down,
            Dir::Right => Dir::Up,
            Dir::Up => Dir::Left,
            Dir::Down => Dir::Right,
        }
    }

    fn right90(&self) -> Dir {
        match self {
            Dir::Left => Dir::Up,
            Dir::Right => Dir::Down,
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
        }
    }
}

impl Map {
    fn from_str(txt: &str) -> Self {
        let mut map = HashMap::new();
        let mut start = Point { i: 0, j: 0 };
        for (i, line) in txt.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                let point = Point {
                    i: i as isize,
                    j: j as isize,
                };
                let tile = match c {
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
            }
        }
        Self { map, start }
    }

    fn moves(&self, pos: Point, dir: Dir) -> Vec<(Dir, isize)> {
        let mut moves = vec![];
        match self.get(pos + dir.as_point()) {
            Tile::Empty | Tile::End => moves.push((dir, 1)),
            _ => {}
        }
        moves.push((dir.left90(), 1000));
        moves.push((dir.right90(), 1000));
        moves
    }

    fn get(&self, pos: Point) -> Tile {
        *self.map.get(&pos).unwrap()
    }

    fn cost_to_end(&self, pos: Point, dir: Dir) -> isize {
        let mut heap = BinaryHeap::new();
        heap.push(Reverse((0, pos, dir)));

        let mut visited = HashMap::new();
        visited.insert((pos, dir), 0);

        while let Some(Reverse((cost, pos, dir))) = heap.pop() {
            if self.get(pos) == Tile::End {
                return cost;
            }

            for (new_dir, new_cost) in self.moves(pos, dir) {
                let new_pos = if dir == new_dir {
                    pos + new_dir.as_point()
                } else {
                    pos
                };
                let new_cost = cost + new_cost;
                if let Some(&best_cost) = visited.get(&(new_pos, new_dir)) {
                    if new_cost > best_cost {
                        continue;
                    }
                }
                visited.insert((new_pos, new_dir), new_cost);
                heap.push(Reverse((new_cost, new_pos, new_dir)));
            }
        }

        panic!("No path to end");
    }

    fn count_all_tiles_on_best_paths(&self, pos: Point, dir: Dir) -> isize {
        let mut heap = BinaryHeap::new();
        heap.push(Reverse((0, vec![pos], dir)));

        let mut visited = HashMap::new();
        visited.insert((pos, dir), 0);

        let mut tiles_on_best_paths: HashSet<Point> = HashSet::new();
        let mut cost_to_end: Option<isize> = None;

        while let Some(Reverse((cost, path, dir))) = heap.pop() {
            // println!("cost: {}, pos: {}, dir: {:?}", cost, pos + Point { i: 1, j: 1 }, dir);
            let pos = *path.last().unwrap();
            if self.get(pos) == Tile::End {
                tiles_on_best_paths.extend(path);
                if cost_to_end.is_none() {
                    cost_to_end = Some(cost);
                } else {
                    assert_eq!(cost, cost_to_end.unwrap());
                }
                continue;
            }

            for (new_dir, new_cost) in self.moves(pos, dir) {
                let new_pos = if dir == new_dir {
                    pos + new_dir.as_point()
                } else {
                    pos
                };
                let new_cost = cost + new_cost;
                if let Some(&best_cost) = visited.get(&(new_pos, new_dir)) {
                    if new_cost > best_cost {
                        continue;
                    }
                }
                if cost_to_end.is_some() && new_cost > cost_to_end.unwrap() {
                    continue;
                }
                let mut new_path = path.clone();
                new_path.push(new_pos);
                visited.insert((new_pos, new_dir), new_cost);
                heap.push(Reverse((new_cost, new_path, new_dir)));
            }
        }

        // self.print_map(&tiles_on_best_paths);

        tiles_on_best_paths.len() as isize
    }

    fn print_map(&self, path: &HashSet<Point>) {
        let mut min_i = isize::MAX;
        let mut max_i = isize::MIN;
        let mut min_j = isize::MAX;
        let mut max_j = isize::MIN;
        for point in self.map.keys() {
            min_i = min_i.min(point.i);
            max_i = max_i.max(point.i);
            min_j = min_j.min(point.j);
            max_j = max_j.max(point.j);
        }

        for i in min_i..=max_i {
            for j in min_j..=max_j {
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

    map.cost_to_end(map.start, Dir::Right)
}

fn part2(txt: &str) -> isize {
    let map = Map::from_str(txt);

    map.count_all_tiles_on_best_paths(map.start, Dir::Right)
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

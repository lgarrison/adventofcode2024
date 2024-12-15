use std::{collections::{HashMap, HashSet}, fs};

use regex::NoExpand;

#[derive(Debug)]
struct Warehouse {
    map: HashMap<Point, usize>,
    objects: Vec<Object>,
    robot: Point,
    part2: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Object {
    kind: ObjectKind,
    pos: Point,
}

impl Object {
    fn from_char(c: char, pos: Point) -> Self {
        let kind = match c {
            '.' => ObjectKind::Empty,
            '#' => ObjectKind::Wall,
            'O' => ObjectKind::Box,
            '@' => ObjectKind::Robot,
            _ => panic!("Invalid object: {}", c),
        };
        Self { kind, pos }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ObjectKind {
    Empty,
    Wall,
    Box,
    Robot,
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

impl Dir {
    fn as_point(&self) -> Point {
        match self {
            Dir::Left => Point { i: 0, j: -1 },
            Dir::Right => Point { i: 0, j: 1 },
            Dir::Up => Point { i: -1, j: 0 },
            Dir::Down => Point { i: 1, j: 0 },
        }
    }

    fn from_char(c: char) -> Self {
        match c {
            '<' => Dir::Left,
            '>' => Dir::Right,
            '^' => Dir::Up,
            'v' => Dir::Down,
            _ => panic!("Invalid direction: {}", c),
        }
    }
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

impl std::ops::AddAssign<Point> for Point {
    fn add_assign(&mut self, other: Point) {
        *self = *self + other;
    }
}

impl std::ops::Sub<Point> for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            i: self.i - other.i,
            j: self.j - other.j,
        }
    }
}

impl Warehouse {
    fn from_str(txt: &str, part2: bool) -> Self {
        let mut map: HashMap<Point, usize> = HashMap::new();
        let mut objects = vec![];
        let mut robot = Point { i: 0, j: 0 };

        for (i, line) in txt.lines().enumerate() {
            for (mut j, c) in line.chars().enumerate() {
                if part2 {
                    j *= 2;
                }
                let pos = Point {
                    i: i as isize,
                    j: j as isize,
                };
                let obj = Object::from_char(c, pos);
                if obj.kind != ObjectKind::Empty {
                    map.insert(pos, objects.len());
                    if part2 && obj.kind != ObjectKind::Robot {
                        map.insert(pos + Point { i: 0, j: 1 }, objects.len());
                    }
                    objects.push(obj);

                    if obj.kind == ObjectKind::Robot {
                        robot = pos;
                    }
                }
            }
        }

        Self {
            map,
            objects,
            robot,
            part2,
        }
    }

    fn neighbors(&self, obj: Object, dir: Dir) -> Vec<Object> {
        let to = obj.pos + dir.as_point();
        let next_obj_poses: Vec<Point> = if self.part2 && obj.kind != ObjectKind::Robot {
            match dir {
                Dir::Left => vec![to],
                Dir::Right => vec![to + dir.as_point()],
                Dir::Up | Dir::Down => vec![to, to + Point { i:0, j: 1 }],
            }
        } else {
            vec![to]
        };

        // uniquify by index
        let indices = next_obj_poses
            .iter()
            .filter_map(|&pos| self.map.get(&pos))
            .cloned()
            .collect::<HashSet<_>>();

        indices.iter().map(|&idx| self.objects[idx]).collect()
    }

    fn can_push(&self, obj: Object, dir: Dir) -> bool {
        let all_neigh_obj = self.neighbors(obj, dir);

        for neigh_obj in all_neigh_obj {
            match neigh_obj.kind {
                ObjectKind::Wall => return false,
                ObjectKind::Box => {
                    if !self.can_push(neigh_obj, dir) {
                        return false;
                    }
                }
                _ => {}
            }
        }

        true
    }

    fn try_push(&mut self, obj: Object, dir: Dir) -> bool {
        // recursively push box in direction
        // returns if actually moved

        if !self.can_push(obj, dir) {
            return false;
        }

        self.push(obj, dir);
        true
    }

    fn push(&mut self, obj: Object, dir: Dir) {
        let all_neigh_obj = self.neighbors(obj, dir);

        for neigh_obj in all_neigh_obj {
            match neigh_obj.kind {
                ObjectKind::Wall => panic!("Invalid push"),
                ObjectKind::Box => self.push(neigh_obj, dir),
                _ => {}
            }
        }

        self.move_one(obj, obj.pos + dir.as_point());
    }

    fn get(&self, pos: Point) -> Option<&Object> {
        if let Some(idx) = self.map.get(&pos) {
            self.objects.get(*idx)
        } else {
            None
        }
    }

    fn get_mut(&mut self, pos: Point) -> Option<&mut Object> {
        if let Some(idx) = self.map.get(&pos) {
            self.objects.get_mut(*idx)
        } else {
            None
        }
    }

    fn move_one(&mut self, obj: Object, to: Point) {
        // move, assuming the destination is empty
        let idx = self.map.remove(&obj.pos).unwrap();
        let wide = self.part2 && obj.kind != ObjectKind::Robot;
        if wide {
            let idx2 = self.map.remove(&(obj.pos + Point { i: 0, j: 1 })).unwrap();
            assert_eq!(idx, idx2);
        }

        self.map.insert(to, idx);
        if wide {
            self.map.insert(to + Point { i: 0, j: 1 }, idx);
        }

        self.objects[idx].pos = to;
    }

    fn score(&self) -> isize {
        self.objects
            .iter()
            .filter(|obj| obj.kind == ObjectKind::Box)
            .map(|obj| 100 * obj.pos.i + obj.pos.j)
            .sum()
    }
}

impl std::fmt::Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut max_i = 0;
        let mut max_j = 0;
        for pos in self.map.keys() {
            max_i = max_i.max(pos.i);
            max_j = max_j.max(pos.j);
        }

        for i in 0..=max_i {
            for j in 0..=max_j {
                let pos = Point { i, j };
                let kind = self
                    .get(pos)
                    .map(|obj| obj.kind)
                    .unwrap_or(ObjectKind::Empty);
                let c = match kind {
                    ObjectKind::Empty => '.',
                    ObjectKind::Wall => '#',
                    ObjectKind::Box => 'O',
                    ObjectKind::Robot => '@',
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn get_moves(txt: &str) -> Vec<Dir> {
    txt.chars()
        .filter(|&c| c != '\n')
        .map(Dir::from_char)
        .collect()
}

fn part1(txt: &str) -> isize {
    let mut parts = txt.split("\n\n");
    let mut warehouse = Warehouse::from_str(parts.next().unwrap(), false);
    println!("{}", warehouse);

    let moves = get_moves(parts.next().unwrap());

    for dir in moves {
        let robot = *warehouse.get(warehouse.robot).unwrap();
        if warehouse.try_push(robot, dir) {
            warehouse.robot += dir.as_point();
        }
    }
    println!("{}", warehouse);
    warehouse.score()
}

fn part2(txt: &str) -> isize {
    let mut parts = txt.split("\n\n");
    let mut warehouse = Warehouse::from_str(parts.next().unwrap(), true);
    println!("{}", warehouse);

    let moves = get_moves(parts.next().unwrap());

    for dir in moves {
        // println!("{:?}", dir);
        // wait for input
        // let mut input = String::new();
        // std::io::stdin().read_line(&mut input).unwrap();
        let robot = *warehouse.get(warehouse.robot).unwrap();
        if warehouse.try_push(robot, dir) {
            warehouse.robot += dir.as_point();
        }
        // println!("{}", warehouse);
    }
    // println!("{}", warehouse);
    warehouse.score()
}

fn main() {
    let day_x = env!("CARGO_BIN_NAME");
    let root = env!("CARGO_MANIFEST_DIR");

    let path = String::from(root) + "/src/" + day_x + "/input.txt";
    // let path = String::from(root) + "/src/" + day_x + "/test1.txt";
    // let path = String::from(root) + "/src/" + day_x + "/test2.txt";
    // let path = String::from(root) + "/src/" + day_x + "/test3.txt";
    let txt = fs::read_to_string(path).unwrap();

    println!("This is {}", day_x);
    // println!("Part 1: {:?}", part1(&txt));
    println!("Part 2: {:?}", part2(&txt));
}

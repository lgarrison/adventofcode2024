#![allow(non_snake_case)]

use std::{collections::{HashMap, HashSet, VecDeque}, fs};

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
    Activate,
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

impl std::ops::Sub<Point> for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            i: self.i - other.i,
            j: self.j - other.j,
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
            Dir::Activate => self,
        }
    }
}

impl std::ops::AddAssign<Dir> for Point {
    fn add_assign(&mut self, other: Dir) {
        match other {
            Dir::Left => self.j -= 1,
            Dir::Right => self.j += 1,
            Dir::Up => self.i -= 1,
            Dir::Down => self.i += 1,
            Dir::Activate => (),
        }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.i, self.j)
    }
}

impl Point {
    fn manhattan(&self) -> i64 {
        self.i.abs() as i64 + self.j.abs() as i64
    }

    // fn mv(&mut self, dir: Dir) {
    //     if match dir {
    //         Dir::Left => self.j > 0,
    //         Dir::Right => self.j < 2,
    //         Dir::Up => self.i > 0,
    //         Dir::Down => self.i < 2,
    //     } {
    //         self += dir;
    //     }
    // }
}

fn char_to_numeric_keypad_loc(c: char) -> Point {
    match c {
        'A' => Point { i: 3, j: 2 },
        '0' => Point { i: 3, j: 1 },
        '1' => Point { i: 2, j: 0 },
        '2' => Point { i: 2, j: 1 },
        '3' => Point { i: 2, j: 2 },
        '4' => Point { i: 1, j: 0 },
        '5' => Point { i: 1, j: 1 },
        '6' => Point { i: 1, j: 2 },
        '7' => Point { i: 0, j: 0 },
        '8' => Point { i: 0, j: 1 },
        '9' => Point { i: 0, j: 2 },
        _ => panic!("Invalid char"),
    }
}

fn numeric_keypad_dist(a: char, b: char) -> Point {
    char_to_numeric_keypad_loc(b) - char_to_numeric_keypad_loc(a)
}

fn numeric_part(code: &str) -> usize {
    code[..code.len() - 1].parse().unwrap()
}

fn transitions(state: &Vec<Point>) -> Vec<Vec<Point>> {
    let mut res = vec![];
    // left
    if state[0].j > 0 {
        let mut new = state.clone();
        new[0] += Dir::Left;
        res.push(new);
    }

    // right
    if state[0].j < 2 {
        let mut new = state.clone();
        new[0] += Dir::Right;
        res.push(new);
    }

    // up
    if state[0].i > 0 {
        let mut new = state.clone();
        new[0] += Dir::Up;
        res.push(new);
    }

    // down
    if state[0].i < 2 {
        let mut new = state.clone();
        new[0] += Dir::Down;
        res.push(new);
    }

    // activate
    let mut new = state.clone();
    activate(&mut new[..]);
    res.push(new);

    res
}

fn activate(state: &mut [Point]) {
    if state.len() <= 1 {
        return;
    }

    let dirpad_left = Point { i: 1, j: 0 };
    let dirpad_right = Point { i: 1, j: 2 };
    let dirpad_up = Point { i: 0, j: 1 };
    let dirpad_down = Point { i: 1, j: 1 };
    let dirpad_a = Point { i: 0, j: 2 };

    if state[0] == dirpad_left {
        if state[1].j > 0 {
            if state.len() == 2 && state[1] == (Point { i: 3, j: 1 }) {
                return;
            }
            state[1] += Dir::Left;
        }
    } else if state[0] == dirpad_right {
        if state[1].j < 2 {
            state[1] += Dir::Right;
        }
    } else if state[0] == dirpad_up {
        if state[1].i > 0 {
            state[1] += Dir::Up;
        }
    } else if state[0] == dirpad_down {
        // if state[1].i < 3 {
        //     state[1] += Dir::Down;
        // }
        if state.len() == 2 && state[1] == (Point { i: 2, j: 0 }) {
            return;
        }
        if state.len() == 2 && state[1].i < 3 {
            state[1] += Dir::Down;
        } else if state[1].i < 2 {
            state[1] += Dir::Down;
        }
    } else if state[0] == dirpad_a {
        activate(&mut state[1..]);
    }
}

type Memo = HashMap<[Vec<Point>;2], usize>;

fn shortest_len(start: &Vec<Point>, end: &Vec<Point>, memo: &mut Memo) -> usize {
    if start == end {
        return 0;
    }
    if let Some(&len) = memo.get(&[start.clone(), end.clone()]) {
        println!("memo hit");
        return len;
    }

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start.clone(), 0));

    while let Some((state, len)) = queue.pop_front() {
        // println!("{:?} -> {}", state, len);
        if state == *end {
            // memo.insert([start.clone(), end.clone()], len);
            return len;
        }

        if visited.contains(&state) {
            continue;
        }

        visited.insert(state.clone());

        for next_state in transitions(&state) {
            queue.push_back((next_state, len + 1));
        }
    }

    panic!("No path found");
}

fn shortest_sequence_len(code: &str, n: usize) -> usize {
    let DIRPAD_A = Point { i: 0, j: 2 };
    let mut state = vec![char_to_numeric_keypad_loc('A')];
    for _ in 0..n - 1 {
        state.insert(0, DIRPAD_A);
    }

    let mut total_len = 0;
    let mut memo = Memo::new();
    for c in code.chars() {
        let mut target_state = state.clone();
        let len = target_state.len();
        target_state[len - 1] = char_to_numeric_keypad_loc(c);
        println!("{} -> {:?}", c, target_state);
        let len = shortest_len(&state, &target_state, &mut memo) + 1;
        state = target_state;
        total_len += len;
    }
    total_len
}

fn part1(txt: &str) -> usize {
    let codes: Vec<&str> = txt.lines().collect();

    // Model the system's state as a [pos; 3]
    // The initial state is [A, A, A]
    // For each digit on the numeric keypad, the target state is
    // [A, A, digit]
    // Transitions are inputs to *our* dirpad.
    // If we activate our dirpad, that triggers the next level to move the *next* level

    codes
        .iter()
        // .take(1)
        .map(|c| shortest_sequence_len(c, 3) 
        * numeric_part(c)
        )
        .inspect(| x| println!("len: {}", x))
        .sum()
}

fn part2(txt: &str) -> usize {
    let codes: Vec<&str> = txt.lines().collect();

    codes
        .iter()
        // .take(1)
        // .map(|c| shortest_sequence_len(c, 3 + 23) 
        .map(|c| shortest_sequence_len(c, 3 + 6)
        * numeric_part(c)
        )
        .inspect(| x| println!("len: {}", x))
        .sum()
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

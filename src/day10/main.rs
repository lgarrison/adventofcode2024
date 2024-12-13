use std::{collections::HashSet, fs, ops::Add};

#[derive(Debug)]
struct Grid {
    grid: Vec<i8>,
    N: isize,
    M: isize,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Debug)]
struct Point {
    i: isize,
    j: isize,
}

impl Point {
    fn new(i: isize, j: isize) -> Self {
        Point { i: i, j: j }
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

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.i, self.j)
    }
}

impl Grid {
    fn from_str(txt: &str) -> Self {
        let M = txt.lines().next().unwrap().len();
        let N = txt.lines().count();
        let grid: Vec<i8> = txt
            .chars()
            .filter_map(|c| c.to_digit(10).and_then(|c| Some(c as i8)))
            .collect();

        assert!(N * M == grid.len());

        Grid {
            grid: grid,
            N: N as isize,
            M: M as isize,
        }
    }

    fn get(&self, p: Point) -> Option<i8> {
        if p.i >= 0 && p.i < self.N && p.j >= 0 && p.j < self.M {
            Some(self.grid[(p.i * self.M + p.j) as usize])
        } else {
            None
        }
    }

    fn moves(&self, p: Point) -> Vec<Point> {
        let h = self.get(p).unwrap();
        [
            Point::new(-1, 0),
            Point::new(1, 0),
            Point::new(0, -1),
            Point::new(0, 1),
        ]
        .iter()
        .filter_map(|&dp| {
            let nextp = p + dp;
            self.get(nextp)
                .and_then(|v| if v == h + 1 { Some(nextp) } else { None })
        })
        .collect()
    }

    fn points(&self) -> Vec<Point> {
        (0..self.N)
            .flat_map(|i| (0..self.M).map(move |j| Point::new(i, j)))
            .collect()
    }

    fn trailheads(&self) -> Vec<Point> {
        self.points()
            .into_iter()
            .filter(|&p| self.get(p).unwrap() == 0)
            .collect()
    }

    fn count_reachable_peaks(&self, p: Point, visited: &mut HashSet<Point>) -> isize {
        if visited.contains(&p) {
            return 0;
        }
        visited.insert(p);

        if self.get(p).unwrap() == 9 {
            return 1;
        }

        self.moves(p)
            .iter()
            // .inspect(|&next| println!("{} -> {}", p, next))
            .map(|&next| self.count_reachable_peaks(next, visited))
            .sum()
    }

    fn count_trails(&self, p: Point) -> isize {
        if self.get(p).unwrap() == 9 {
            return 1;
        }

        self.moves(p)
            .iter()
            // .inspect(|&next| println!("{} -> {}", p, next))
            .map(|&next| self.count_trails(next))
            .sum()
    }
}

fn score_p1(grid: &Grid) -> isize {
    grid.trailheads()
        .iter()
        .map(|&t| {
            let mut visited = HashSet::new();
            grid.count_reachable_peaks(t, &mut visited)
        })
        // .inspect(|c| println!("{} reachable", c))
        .sum()
}

fn score_p2(grid: &Grid) -> isize {
    grid.trailheads()
        .iter()
        .map(|&t| grid.count_trails(t))
        // .inspect(|c| println!("{} reachable", c))
        .sum()
}

fn part1(txt: &str) -> isize {
    let grid = Grid::from_str(txt);
    score_p1(&grid)
}

fn part2(txt: &str) -> isize {
    let grid = Grid::from_str(txt);
    score_p2(&grid)
}

fn main() {
    let day_x = env!("CARGO_BIN_NAME");
    let root = env!("CARGO_MANIFEST_DIR");

    let path = String::from(root) + "/src/" + day_x + "/input.txt";
    // let path = String::from(root) + "/src/" + day_x + "/test1.txt";
    // let path = String::from(root) + "/src/" + day_x + "/test2.txt";
    let txt = fs::read_to_string(path).unwrap();

    println!("This is {}", day_x);
    println!("Part 1: {:?}", part1(&txt));
    println!("Part 2: {:?}", part2(&txt));
}

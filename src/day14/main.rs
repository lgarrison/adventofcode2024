use std::fs;

#[derive(Debug)]
struct Board {
    robots: Vec<Robot>,
    domain: Point,
    step: isize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Robot {
    pos: Point,
    vel: Point,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    i: isize,
    j: isize,
}

impl Point {
    fn rem_euclid(self, domain: Point) -> Point {
        Point {
            i: (self.i + domain.i).rem_euclid(domain.i),
            j: (self.j + domain.j).rem_euclid(domain.j),
        }
    }

    fn pow(self, n: isize) -> Point {
        Point {
            i: self.i.pow(n as u32),
            j: self.j.pow(n as u32),
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

impl std::ops::Mul<isize> for Point {
    type Output = Point;

    fn mul(self, n: isize) -> Point {
        Point {
            i: self.i * n,
            j: self.j * n,
        }
    }
}

impl std::ops::Div<isize> for Point {
    type Output = Point;

    fn div(self, n: isize) -> Point {
        Point {
            i: self.i / n,
            j: self.j / n,
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

impl std::ops::AddAssign<Point> for Point {
    fn add_assign(&mut self, other: Point) {
        self.i += other.i;
        self.j += other.j;
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.j, self.i)
    }
}

impl Board {
    fn from_str(txt: &str, domain: Point) -> Self {
        // format:
        // p=0,4 v=3,-3

        let robots = txt
            .lines()
            .map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                let pos_parts: Vec<&str> = parts[0].split(',').collect();
                let vel_parts: Vec<&str> = parts[1].split(',').collect();

                let pos = Point {
                    j: pos_parts[0][2..].parse().unwrap(),
                    i: pos_parts[1].parse().unwrap(),
                };

                let vel = Point {
                    j: vel_parts[0][2..].parse().unwrap(),
                    i: vel_parts[1].parse().unwrap(),
                };

                Robot { pos, vel }
            })
            .collect();
        
        Board { robots, domain, step: 0 }
    }

    fn step(&mut self, n: isize) {
        for robot in self.robots.iter_mut() {
            robot.pos = (robot.pos + robot.vel * n).rem_euclid(self.domain);
        }
        self.step += n;
    }

    fn score(&self) -> isize {
        // count robots in each quadrant and multiply
        let mut quad = [0; 4];
        for robot in self.robots.iter() {
            if robot.pos.i == self.domain.i / 2 || robot.pos.j == self.domain.j / 2 {
                continue;
            }
            let i = if robot.pos.i < self.domain.i / 2 { 0 } else { 1 };
            let j = if robot.pos.j < self.domain.j / 2 { 0 } else { 1 };
            quad[(i * 2 + j) as usize] += 1;
        }

        quad.iter().product()
    }

    fn print(&self) {
        let mut board = vec![vec![0; self.domain.j as usize]; self.domain.i as usize];
        for robot in self.robots.iter() {
            board[robot.pos.i as usize][robot.pos.j as usize] += 1;
        }

        for i in 0..self.domain.i {
            for j in 0..self.domain.j {
                let count = board[i as usize][j as usize];
                if count == 0 {
                    print!(".");
                } else {
                    print!("{}", count);
                }
            }
            println!();
        }
    }

    fn variance(&self) -> Point {
        // actually N^2 * var
        let n = self.robots.len() as isize;
        let sum = self.robots.iter().fold(Point { i: 0, j: 0 }, |acc, robot| acc + robot.pos);
        self.robots.iter().fold(Point { i: 0, j: 0 }, |acc, robot| acc + (robot.pos * n - sum).pow(2))
    }
}


fn part1(txt: &str, domain: Point) -> isize {
    let mut board = Board::from_str(txt, domain);
    board.step(100);
    board.score()
}

fn part2(txt: &str, domain: Point) -> isize {
    let mut board = Board::from_str(txt, domain);


    println!("Step: {}, Variance: {}", board.step, board.variance());
    let mut minvar = board.variance();
    let mut minvarstep = Point{ i: 0, j: 0 };
    for _ in 0..200 {
        board.step(1);
        let newvar = board.variance();
        if newvar.i < minvar.i {
            minvar.i = newvar.i;
            minvarstep.i = board.step;
            // board.print();
        }
        if newvar.j < minvar.j {
            minvar.j = newvar.j;
            minvarstep.j = board.step;
            // board.print();
        }
        // println!("Step: {}, Variance: {}", board.step, board.variance());
    }
    println!("Min Variance: {}", minvar);

    let steps = minvarstep.j + (51 * (minvarstep.i - minvarstep.j) % board.domain.i) * board.domain.j;

    let mut board = Board::from_str(txt, domain);
    board.step(steps);
    board.print();

    steps
}

fn main() {
    let day_x = env!("CARGO_BIN_NAME");
    let root = env!("CARGO_MANIFEST_DIR");

    let path = String::from(root) + "/src/" + day_x + "/input.txt"; let domain = Point{ i: 103, j: 101 };
    // let path = String::from(root) + "/src/" + day_x + "/test1.txt"; let domain = Point{ i: 7, j: 11 };
    let txt = fs::read_to_string(path).unwrap();

    println!("This is {}", day_x);
    println!("Part 1: {:?}", part1(&txt, domain));
    println!("Part 2: {:?}", part2(&txt, domain));
}

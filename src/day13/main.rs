use regex::Regex;
use std::fs;

#[allow(non_snake_case)]
#[derive(Debug)]
struct Machine {
    prize: Point,
    matrix: Matrix,
}

#[derive(Debug)]
struct Matrix {
    a: [[isize; 2]; 2],
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    i: isize,
    j: isize,
}

#[derive(Debug)]
struct Arcade {
    machines: Vec<Machine>,
    part2: bool,
}

impl Point {
    fn from_array(arr: [isize; 2]) -> Self {
        Point {
            i: arr[0],
            j: arr[1],
        }
    }
}

impl std::ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point {
            i: self.i + rhs.i,
            j: self.j + rhs.j,
        }
    }
}

impl std::ops::Div<isize> for Point {
    type Output = Point;

    fn div(self, rhs: isize) -> Point {
        Point {
            i: self.i / rhs,
            j: self.j / rhs,
        }
    }
}

impl std::ops::Rem<isize> for Point {
    type Output = Point;

    fn rem(self, rhs: isize) -> Point {
        Point {
            i: self.i % rhs,
            j: self.j % rhs,
        }
    }
}

#[allow(non_snake_case)]
impl Machine {
    fn from_str(txt: &str, part2: bool) -> Self {
        let re = Regex::new(r"X[+=](\d+), Y[+=](\d+)").unwrap();
        let mut caps = re.captures_iter(txt).map(|c| {
            [
                c[1].parse::<isize>().unwrap(),
                c[2].parse::<isize>().unwrap(),
            ]
        });

        let off = if part2 {
            Point {
                i: 10000000000000,
                j: 10000000000000,
            }
        } else {
            Point { i: 0, j: 0 }
        };
        let A = Point::from_array(caps.next().unwrap());
        let B = Point::from_array(caps.next().unwrap());
        let prize = Point::from_array(caps.next().unwrap()) + off;

        let matrix = Matrix {
            a: [[A.i, B.i], [A.j, B.j]],
        };

        Machine { prize, matrix }
    }

    fn tokens_to_solve(&self) -> isize {
        if let Some(sol) = self.matrix.solve(&self.prize) {
            3 * sol.i + sol.j
        } else {
            0
        }
    }
}

impl Matrix {
    fn solve(&self, rhs: &Point) -> Option<Point> {
        let inv: Matrix = self.inverse_nodet();

        let sol = Point {
            i: inv.a[0][0] * rhs.i + inv.a[0][1] * rhs.j,
            j: inv.a[1][0] * rhs.i + inv.a[1][1] * rhs.j,
        };

        let det = self.det();
        if sol % det == (Point { i: 0, j: 0 }) {
            Some(sol / det)
        } else {
            None
        }
    }

    fn inverse_nodet(&self) -> Matrix {
        Matrix {
            a: [[self.a[1][1], -self.a[0][1]], [-self.a[1][0], self.a[0][0]]],
        }
    }

    fn det(&self) -> isize {
        self.a[0][0] * self.a[1][1] - self.a[0][1] * self.a[1][0]
    }
}

impl Arcade {
    fn from_str(txt: &str, part2: bool) -> Self {
        Arcade {
            machines: txt
                .split("\n\n")
                .map(|m| Machine::from_str(m, part2))
                .collect(),
            part2,
        }
    }

    fn tokens_to_solve(&self) -> isize {
        self.machines.iter().map(Machine::tokens_to_solve).sum()
    }
}

fn part1(txt: &str) -> isize {
    let arcade = Arcade::from_str(txt, false);
    arcade.tokens_to_solve()
}

fn part2(txt: &str) -> isize {
    let arcade = Arcade::from_str(txt, true);
    arcade.tokens_to_solve()
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

use std::fs;

#[derive(Debug)]
struct Grid {
    rows: usize,
    cols: usize,
    grid: Vec<isize>,
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

const DIRS: [Dir; 8] = [Dir::N, Dir::NE, Dir::E, Dir::SE, Dir::S, Dir::SW, Dir::W, Dir::NW];

impl Dir {
    fn mirror(&self) -> Self {
        match self {
            Dir::N => Dir::S,
            Dir::NE => Dir::SW,
            Dir::E => Dir::W,
            Dir::SE => Dir::NW,
            Dir::S => Dir::N,
            Dir::SW => Dir::NE,
            Dir::W => Dir::E,
            Dir::NW => Dir::SE,
        }
    }
}

impl Grid {
    fn from_str(txt: &str) -> Self {
        let rows = txt.lines().count();
        let cols = txt.lines().next().unwrap().chars().count();
        let mut grid = vec![0; rows * cols];
        for (i, line) in txt.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                grid[i * cols + j] = match c {
                    'X' => 1,
                    'M' => 2,
                    'A' => 3,
                    'S' => 4,
                    _ => panic!("Invalid character"),
                }
            }
        }
        Grid { rows, cols, grid }
    }

    fn get(&self, i: usize, j: usize) -> isize {
        self.grid[i * self.cols + j]
    }

    fn set(&mut self, i: usize, j: usize, val: isize) {
        self.grid[i * self.cols + j] = val;
    }

    fn neigh(&self, i: usize, j: usize, dir: Dir, dist: usize) -> isize {
        let (mut i, mut j) = (i as isize, j as isize);
        match dir {
            Dir::N => i -= dist as isize,
            Dir::NE => {
                i -= dist as isize;
                j += dist as isize;
            }
            Dir::E => j += dist as isize,
            Dir::SE => {
                i += dist as isize;
                j += dist as isize;
            }
            Dir::S => i += dist as isize,
            Dir::SW => {
                i += dist as isize;
                j -= dist as isize;
            }
            Dir::W => j -= dist as isize,
            Dir::NW => {
                i -= dist as isize;
                j -= dist as isize;
            }
        }
        if i < 0 || i >= self.rows as isize || j < 0 || j >= self.cols as isize {
            0
        } else {
            self.get(i as usize, j as usize)
        }
    }

    fn process(&mut self) -> bool {
        // For each char, loop over dirs.
        // If the char in that dir is adjacent to us in the word XMAS,
        // check the mirror char. If that char works too, keep this char
        // and move on. Otherwise, set this char to 0.
        let mut changed = false;

        for i in 0..self.rows {
            for j in 0..self.cols {
                let c = self.get(i, j);
                if c > 0 {
                    let mut keep = false;
                    for dir in vec![Dir::N, Dir::NE, Dir::E, Dir::SE, Dir::S, Dir::SW, Dir::W, Dir::NW] {
                        let d = self.neigh(i, j, dir, 1);
                        if d > 0 && c.abs_diff(d) == 1 {
                            if c == 1 || c == 4 {
                                keep = true;
                                break;
                            }
                            let m = self.neigh(i, j, dir.mirror(), 1);
                            if m > 0 && (c - d) == (m - c) {
                                keep = true;
                                break;
                            }
                        }
                    }
                    if !keep {
                        self.set(i, j, 0);
                        changed = true;
                    }
                }
            }
        }
        changed
    }

    fn process2(&self) -> usize {
        // For each X, check each dir. If the next letter is A, check the next
        // and the next in the same dir.

        let mut count = 0;

        for i in 0..self.rows {
            for j in 0..self.cols {
                let x = self.get(i, j);
                if x == 1 {
                    for dir in DIRS {
                        let m = self.neigh(i, j, dir, 1);
                        if m == 2 {
                            let a = self.neigh(i, j, dir, 2);
                            if a == 3 {
                                let s = self.neigh(i, j, dir, 3);
                                if s == 4 {
                                    count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        count
    }

    fn process_part2(&self) -> usize {
        let mut count = 0;

        for i in 0..self.rows {
            for j in 0..self.cols {
                let a = self.get(i, j);
                if a == 3 {
                    if [Dir::NW, Dir::NE].iter().all(|&dir| {
                        let ms = self.neigh(i, j, dir, 1);
                        let sm = self.neigh(i, j, dir.mirror(), 1);
                        (ms == 2 && sm == 4) || (ms == 4 && sm == 2)
                    }) {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    fn nonzero(&self) -> usize {
        self.grid.iter().filter(|&&x| x > 0).count()
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..self.rows {
            for j in 0..self.cols {
                let c = self.get(i, j);
                let c = match c {
                    0 => '.',
                    1 => 'X',
                    2 => 'M',
                    3 => 'A',
                    4 => 'S',
                    _ => panic!("Invalid character"),
                };
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn part1(txt: &str) -> usize {
    let grid = Grid::from_str(txt);

    // println!("{}", grid);

    // loop {
    //     if !grid.process() {
    //         break;
    //     }
    // }

    // println!("{}", grid);

    // grid.nonzero()

    grid.process2()
}

fn part2(txt: &str) -> usize {
    let grid = Grid::from_str(txt);
    grid.process_part2()
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

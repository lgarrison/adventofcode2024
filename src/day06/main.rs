use std::fs;

#[derive(Debug)]
struct Grid {
    rows: usize,
    cols: usize,
    grid: Vec<isize>,
    pos: [usize; 2],
    dir: Dir,
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    fn turn_right(&self) -> Self {
        match self {
            Dir::N => Dir::E,
            Dir::E => Dir::S,
            Dir::S => Dir::W,
            Dir::W => Dir::N,
        }
    }

    fn get_step(&self) -> [isize; 2] {
        match self {
            Dir::N => [-1, 0],
            Dir::E => [0, 1],
            Dir::S => [1, 0],
            Dir::W => [0, -1],
        }
    }

    fn marker(&self) -> isize {
        match self {
            Dir::N => 1 << 1,
            Dir::E => 1 << 2,
            Dir::S => 1 << 3,
            Dir::W => 1 << 4,
        }
    }
}

impl Grid {
    fn from_str(txt: &str) -> Self {
        let rows = txt.lines().count();
        let cols = txt.lines().next().unwrap().chars().count();
        let mut grid = vec![0; rows * cols];
        let mut guard_pos = [0; 2];
        for (i, line) in txt.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                grid[i * cols + j] = match c {
                    '.' => 0,
                    '#' => 1,
                    '^' => {guard_pos = [i, j]; 0},
                    _ => panic!("Invalid character"),
                }
            }
        }
        Grid { rows, cols, grid, pos: guard_pos, dir: Dir::N }
    }

    fn guard_cast(&mut self) -> isize {
        let mut left = false;
        let step = self.dir.get_step();
        let marker = self.dir.marker();
        loop {
            let val = self.get(self.pos[0], self.pos[1]);
            if (val & marker) != 0 {
                return 2;
            }

            self.set(self.pos[0], self.pos[1], val | marker);

            if (self.pos[0] == 0 && step[0] == -1) ||
                (self.pos[0] == self.rows - 1 && step[0] == 1) ||
                (self.pos[1] == 0 && step[1] == -1) ||
                (self.pos[1] == self.cols - 1 && step[1] == 1) {
                left = true;
                break;
            }

            if self.get((self.pos[0] as isize + step[0]) as usize, (self.pos[1] as isize + step[1]) as usize) == 1 {
                break;
            }
            
            self.pos[0] = (self.pos[0] as isize + step[0]) as usize;
            self.pos[1] = (self.pos[1] as isize + step[1]) as usize;
        }
        if left { 1 } else { 0 }
    }

    fn get(&self, i: usize, j: usize) -> isize {
        self.grid[i * self.cols + j]
    }

    fn set(&mut self, i: usize, j: usize, val: isize) {
        self.grid[i * self.cols + j] = val;
    }
}

fn part1(txt: &str) -> i64 {
    let mut grid = Grid::from_str(txt);

    loop {
        let ret = grid.guard_cast();
        // println!("{:?}", ret);
        // break;
        if ret == 1 {
            break;
        }
        grid.dir = grid.dir.turn_right();
    }

    grid.grid.iter().filter(|&&x| x >= 2).count() as i64
}

fn part2(txt: &str) -> i64 {
    let mut _grid = Grid::from_str(txt);

    (0.._grid.rows).flat_map(|i| {
        println!("{:?}", i);
        (0.._grid.cols).map(move |j| {
            let mut grid = Grid::from_str(txt);
            grid.set(i, j, 1);
            loop {
                let ret = grid.guard_cast();
                if ret == 1 {
                    return 0;
                } else if ret == 2 {
                    return 1;
                }
                grid.dir = grid.dir.turn_right();
            }
        })
    }).sum::<isize>() as i64
}
fn main() {
    let day_x = env!("CARGO_BIN_NAME");
    let root = env!("CARGO_MANIFEST_DIR");

    // let path = String::from(root) + "/src/" + day_x + "/input.txt";
    let path = String::from(root) + "/src/" + day_x + "/test1.txt";
    let txt = fs::read_to_string(path).unwrap();

    println!("This is {}", day_x);
    println!("Part 1: {:?}", part1(&txt));
    println!("Part 2: {:?}", part2(&txt));
}

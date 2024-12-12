use std::{collections::HashSet, fs};

#[allow(non_snake_case)]
struct Grid {
    grid: Vec<char>,
    N: isize,
    M: isize,
}

impl Grid {
    fn from_str(txt: &str) -> Self {
        let grid = txt.chars()
            .filter(|&c| c != '\n').collect();
        let n = txt.lines().count() as isize;
        let m = txt.lines().next().unwrap().len() as isize;
        Grid { grid, N: n, M: m }
    }

    fn get(&self, i: isize, j: isize) -> Option<char> {
        if i < 0 || i >= self.N || j < 0 || j >= self.M {
            None
        } else {
            Some(self.grid[(i * self.M + j) as usize])
        }
    }

    fn flood(&self, i: isize, j: isize, seen: &mut HashSet<[isize; 2]>, discount: bool) -> Option<[isize; 2]> {
        if seen.contains(&[i, j]) {
            return None;
        }
        if let Some(c) = self.get(i, j) {
            seen.insert([i, j]);
            let mut area = 1;
            
            let mut perimeter = 4;
            for [di, dj] in [[-1isize, 0], [1, 0], [0, -1], [0, 1]] {
                let mut lostedge = false;
                if let Some(d) = self.get(i + di, j + dj) {
                    if d == c {
                        perimeter -= 1;
                        lostedge = true;
                        if let Some(ap) = self.flood(i + di, j + dj, seen, discount){
                            area += ap[0];
                            perimeter += ap[1];
                        }
                    }
                }
                if discount && !lostedge {
                    // Part 2
                    // If this square is the start of a straight edge, count the side.
                    // This means we need to decrement perimeter if we are a continuation.

                    // first, look one square over 90 degrees counterclockwise
                    let dd = if dj == 0 { [0,di] } else { [-dj,0] };

                    if let Some(e) = self.get(i + dd[0], j + dd[1]) {
                        // if that square matches ours, we might need to lose the edge. keep checking.
                        if e == c {
                            // now, check one more square over, in the direction of the edge
                            if let Some(f) = self.get(i + dd[0] + di, j + dd[1] + dj) {
                            
                                // if that square does not match ours, then we know that our neighbor
                                // already has an edge going. lose our edge.
                                if c != f {
                                    perimeter -= 1;
                                }
                            } else {
                                // edge of the board
                                perimeter -= 1;
                            }
                        }
                    }
                }
            }
            return Some([area, perimeter]);
        } else {
            return None;
        }
    }

    fn flood_all(&self, discount: bool) -> isize {
        let mut seen = HashSet::new();
        let mut areas = Vec::new();
        for i in 0..self.N {
            for j in 0..self.M {
                if let Some(ap) = self.flood(i, j, &mut seen, discount) {
                    // println!("Region {}: area {}, perimeter {}", self.get(i, j).unwrap(), ap[0], ap[1]);
                    areas.push(ap);
                }
            }
        }
        areas.iter().map(|[a, p]| a * p).sum()
    }
}

fn part1(txt: &str) -> isize {
    let grid = Grid::from_str(txt);
    grid.flood_all(false)
}

fn part2(txt: &str) -> isize {
    let grid = Grid::from_str(txt);
    grid.flood_all(true)
}

fn main() {
    let day_x = env!("CARGO_BIN_NAME");
    let root = env!("CARGO_MANIFEST_DIR");

    let path = String::from(root) + "/src/" + day_x + "/input.txt";
    // let path = String::from(root) + "/src/" + day_x + "/test1.txt";
    // let path = String::from(root) + "/src/" + day_x + "/test2.txt";
    // let path = String::from(root) + "/src/" + day_x + "/test3.txt";
    // let path = String::from(root) + "/src/" + day_x + "/test4.txt";
    // let path = String::from(root) + "/src/" + day_x + "/test5.txt";
    let txt = fs::read_to_string(path).unwrap();

    println!("This is {}", day_x);
    println!("Part 1: {:?}", part1(&txt));
    println!("Part 2: {:?}", part2(&txt));
}

#![allow(non_snake_case)]

use core::panic;
use std::fs;

#[derive(Debug, Clone, Copy)]
struct Computer {
    A: isize,
    B: isize,
    C: isize,
    ip: usize,
}

impl Computer {
    fn from_str(txt: &str) -> Self {
        let reg: Vec<isize> = txt
            .lines()
            .map(|l| l.split_whitespace().last().unwrap().parse().unwrap())
            .collect();
        Computer {
            A: reg[0],
            B: reg[1],
            C: reg[2],
            ip: 0,
        }
    }

    fn exec(&mut self, prog: &Vec<isize>) -> Vec<isize> {
        // println!("\nTrying with A = {}", self.A);
        // println!("{:?}", prog);
        // [2, 4,
        // B = A & 0b111 [lower 3 of A]
        // 1, 6,
        // B = B ^ 0b110 [lower 3 of A]
        // 7, 5,
        // C = A >> B [C has all bits of A except the lower 0-7]
        // 4, 6
        // B = B ^ C [B's bits (lower 3 of A) now mixed with some higher bits]
        // 1, 4,
        // B = B ^ 0b100 [B now has up to 10 lower bits of A]
        // 5, 5,
        // output B & 0b111 [output: 2; input: lower 3 bits need to be 0b010]
        // 0, 3,
        // A = A >> 3
        // 3, 0]
        // jump to 0
        let mut out = Vec::new();
        out.reserve(prog.len());
        loop {
            // println!("{:?} ", self);
            if self.ip >= prog.len() {
                break;
            }
            let opcode = prog[self.ip];
            let operand = match opcode {
                0 | 2 | 5 | 6 | 7 => self.combo(prog[self.ip + 1]),
                1 | 3 | 4 => prog[self.ip + 1],
                _ => panic!("Unknown opcode"),
            };
            // println!("{:?} {:?}", opcode, operand);
            match opcode {
                0 => self.A = self.adv(operand),
                1 => self.B = self.bxl(operand),
                2 => self.B = self.bst(operand),
                3 => {
                    self.ip = self.jnz(operand);
                    continue;
                }
                4 => self.B = self.bxc(),
                5 => {
                    let newout = self.out(operand);
                    if newout != prog[out.len()] {
                        break;
                    }
                    // println!("Output: {}", newout);
                    out.push(newout)
                }
                6 => self.B = self.adv(operand),
                7 => self.C = self.adv(operand),
                _ => panic!("Unknown opcode"),
            }

            self.ip += 2;
        }

        out
    }

    fn adv(&self, operand: isize) -> isize {
        // self.A / (1 << operand)
        self.A >> operand
    }

    fn bxl(&self, operand: isize) -> isize {
        self.B ^ operand
    }

    fn bst(&self, operand: isize) -> isize {
        operand & 0x7
    }

    fn jnz(&self, operand: isize) -> usize {
        if self.A != 0 {
            operand as usize
        } else {
            self.ip + 2
        }
    }

    fn bxc(&self) -> isize {
        self.B ^ self.C
    }

    fn out(&self, operand: isize) -> isize {
        operand & 0x7
    }

    fn combo(&self, operand: isize) -> isize {
        match operand {
            0..=3 => operand,
            4 => self.A,
            5 => self.B,
            6 => self.C,
            _ => panic!("Unknown operand"),
        }
    }
}

fn compute_one(A: isize) -> isize {
    let mut B = A & 0b111;
    B = B ^ 0b110;
    let C = A >> B;
    B = B ^ C;
    B = B ^ 0b100;
    B & 0b111
}

fn find_A(A: isize, depth: isize, prog: &Vec<isize>) -> Option<isize> {
    if depth < 0 {
        return Some(A);
    }
    for i in 0..8 {
        println!("Trying with A = {}, i = {}", A | i, i);
        let new_A = (A << 3) | i;
        if compute_one(new_A) == prog[depth as usize] {
            if let Some(x) = find_A(new_A, depth - 1, prog) {
                return Some(x);
            }
        }
    }
    None
}

fn get_prog(txt: &str) -> Vec<isize> {
    txt.split_ascii_whitespace()
        .last()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

fn part1(txt: &str) -> String {
    let mut parts = txt.split("\n\n");
    let mut comp = Computer::from_str(parts.next().unwrap());
    let prog: Vec<isize> = get_prog(parts.next().unwrap());

    let out = comp.exec(&prog);

    out.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn part2(txt: &str) -> isize {
    let mut parts = txt.split("\n\n");
    parts.next();
    let prog: Vec<isize> = get_prog(parts.next().unwrap());
    find_A(0, prog.len() as isize - 1, &prog).unwrap()
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

#![allow(non_snake_case)]

use std::{collections::HashMap, fs};

#[derive(Debug)]
enum Gate {
    AND,
    OR,
    XOR,
}

type Wires = HashMap<String, u8>;
type Gates = HashMap<String, (Gate, String, String)>;

impl Gate {
    fn eval(&self, a: u8, b: u8) -> u8 {
        match self {
            Gate::AND => a & b,
            Gate::OR => a | b,
            Gate::XOR => a ^ b,
        }
    }
}

// example input
// x00: 1
// x01: 1
// x02: 1
// y00: 0
// y01: 1
// y02: 0
//
// x00 AND y00 -> z00
// x01 XOR y01 -> z01
// x02 OR y02 -> z02

fn parse(txt: &str) -> (Wires, Gates) {
    let mut wires = Wires::new();
    let mut gates = Gates::new();

    let parts = txt.split("\n\n").collect::<Vec<&str>>();

    for line in parts[0].lines() {
        let parts = line.split(": ").collect::<Vec<&str>>();
        let key = parts[0];
        let value = parts[1].parse::<u8>().unwrap();
        wires.insert(key.to_string(), value);
    }

    for line in parts[1].lines() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let a = parts[0];
        let b = parts[2];
        let c = parts[4];
        let gate = match parts[1] {
            "AND" => Gate::AND,
            "OR" => Gate::OR,
            "XOR" => Gate::XOR,
            _ => panic!("Unknown gate"),
        };
        gates.insert(c.to_string(), (gate, a.to_string(), b.to_string()));
    }

    (wires, gates)
}

fn get_output(wires: &mut Wires, gates: &Gates, wire: &str) -> u8 {
    if let Some(value) = wires.get(wire) {
        return *value;
    }

    if let Some((gate, a, b)) = gates.get(wire) {
        let a = get_output(wires, gates, a);
        let b = get_output(wires, gates, b);
        let value = gate.eval(a, b);
        wires.insert(wire.to_string(), value);
        return value;
    }

    panic!("Unknown key");
}

fn part1(txt: &str) -> u64 {
    let (mut wires, gates) = parse(txt);

    let mut zwires = gates
        .keys()
        .filter(|&x| x.starts_with("z"))
        .cloned()
        .collect::<Vec<String>>();
    zwires.sort();

    let mut output = 0u64;
    for (i, zwire) in zwires.iter().enumerate() {
        let value = get_output(&mut wires, &gates, &zwire);
        println!("{}: {}", zwire, value);
        output |= (value as u64) << i;
    }

    output
}

fn part2(txt: &str) -> u64 {

    for i in 0..45 {
        let (mut wires, gates) = parse(txt);

        // println!("{:?}", wires);
        // set all wires to 0
        let keys: Vec<String> = wires.keys().cloned().collect();
        for key in keys {
            wires.insert(key, 0);
        }

        println!("Setting x{:02} to 1", i);
        wires.insert(format!("x{:02}", i), 1);
        // wires.insert(format!("y{:02}", i), 1);

        let mut zwires = gates
            .keys()
            .filter(|&x| x.starts_with("z"))
            .cloned()
            .collect::<Vec<String>>();
        zwires.sort();

        let mut output = 0u64;
        for (i, zwire) in zwires.iter().enumerate() {
            let value = get_output(&mut wires, &gates, &zwire);
            // println!("{}: {}", zwire, value);
            output |= (value as u64) << i;
        }

        println!("Output: 0x{:12x}", output);
    }

    // x06, x25, x31, x37
    // y06 AND x06 -> z06
    // // y25 AND x25 -> tnt
    // // y25 XOR x25 -> qmd
    // // y31 XOR x31 -> vkh
    // // y31 AND x31 -> hgw
    // // y37 AND x37 -> vbq
    // // x37 XOR y37 -> vqv
    // swj XOR rjv -> hwk
    // gqc XOR vqv -> cgr
    // vkh XOR dtq -> hpc
    // rgn OR mfp -> z45
    // gqc AND vqv -> z37
    // mjr OR hgw -> z31

    // output from x?? AND y?? goes to OR


    // cgr,hpc,hwk,qmd,*tnt,*z06,z31,z37

    // output
    0
}

fn main() {
    let day_x = env!("CARGO_BIN_NAME");
    let root = env!("CARGO_MANIFEST_DIR");

    let path = String::from(root) + "/src/" + day_x + "/input.txt";
    // let path = String::from(root) + "/src/" + day_x + "/test1.txt";
    // let path = String::from(root) + "/src/" + day_x + "/test2.txt";
    let txt = fs::read_to_string(path).unwrap();

    println!("This is {}", day_x);
    // println!("Part 1: {:?}", part1(&txt));
    println!("Part 2: {:?}", part2(&txt));
}

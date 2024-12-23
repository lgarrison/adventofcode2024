#![allow(non_snake_case)]

use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug)]
struct Graph {
    edges: HashMap<String, HashSet<String>>,
}

impl Graph {
    fn from_txt(txt: &str) -> Self {
        let mut edges = HashMap::new();
        for line in txt.lines() {
            let mut parts = line.split("-");
            let a = parts.next().unwrap().to_string();
            let b = parts.next().unwrap().to_string();
            edges
                .entry(a.clone())
                .or_insert(HashSet::new())
                .insert(b.clone());
            edges
                .entry(b.clone())
                .or_insert(HashSet::new())
                .insert(a.clone());
            // self edges
            edges
                .entry(a.clone())
                .or_insert(HashSet::new())
                .insert(b.clone());
            edges
                .entry(b.clone())
                .or_insert(HashSet::new())
                .insert(a.clone());
        }
        Graph { edges }
    }

    fn find_groups_of_three(&self) -> isize {
        // find all groups of three
        // and count how many have the letter 't'

        let mut count = 0;
        // for all triples, check if they are connected
        let nodes: Vec<String> = self.edges.keys().map(|x| x.clone()).collect();
        for (i,a) in nodes.iter().enumerate() {
            for (j,b) in nodes[i..].iter().enumerate() {
                for c in nodes[(i + j)..].iter() {
                    if self.has_edge(a, b) && self.has_edge(b, c) && self.has_edge(c, a) {
                        if a.starts_with("t") || b.starts_with("t") || c.starts_with("t") {
                            // println!("{} {} {}", a, b, c);
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }

    fn find_largest_clique(&self) -> Vec<String> {
        fn is_clique(graph: &Graph, nodes: &[String]) -> bool {
            for i in 0..nodes.len() {
                for j in i+1..nodes.len() {
                    if !graph.has_edge(&nodes[i], &nodes[j]) {
                        return false;
                    }
                }
            }
            true
        }
    
        fn backtrack(graph: &Graph, all_nodes: &[String], start: usize, current: &mut Vec<String>, best: &mut Vec<String>) {
            if current.len() > best.len() {
                *best = current.clone();
            }
            for i in start..all_nodes.len() {
                current.push(all_nodes[i].clone());
                if is_clique(graph, current) {
                    backtrack(graph, all_nodes, i+1, current, best);
                }
                current.pop();
            }
        }
    
        let all_nodes: Vec<String> = self.edges.keys().cloned().collect();
        let mut best = vec![];
        backtrack(self, &all_nodes, 0, &mut vec![], &mut best);
        best
    }

    fn has_edge(&self, a: &str, b: &str) -> bool {
        self.edges[a].contains(b)
    }
}

fn part1(txt: &str) -> isize {
    let graph = Graph::from_txt(txt);
    // println!("{:?}", graph);

    graph.find_groups_of_three()
}

fn part2(txt: &str) -> String {
    let graph = Graph::from_txt(txt);
    let clique = graph.find_largest_clique();
    
    let mut sorted_clique = clique.clone();
    sorted_clique.sort();
    sorted_clique.join(",")
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

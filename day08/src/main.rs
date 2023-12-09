use core::panic;
use std::{collections::HashMap, io::stdin};

use regex::Regex;

fn main() {
    let mut directions = String::new();
    stdin().read_line(&mut directions).unwrap();
    let directions = directions.trim().to_owned();
    let re = Regex::new(r"([0-9A-Z]+) = \(([0-9A-Z]+), ([0-9A-Z]+)\)").unwrap();
    let mut graph = HashMap::new();
    for line in stdin().lines().skip(1) {
        let line = line.unwrap();
        let m = re.captures(&line).unwrap();
        graph.insert(m[1].to_owned(), (m[2].to_owned(), m[3].to_owned()));
    }
    for entry in &graph {
        println!("{} -> {},{}", entry.0, entry.1 .0, entry.1 .1);
    }
    let mut nodes: Vec<_> = graph
        .keys()
        .filter(|x| x.ends_with('A'))
        .map(|x| x.to_owned())
        .collect();
    let mut answers = Vec::new();
    for node in nodes {
        let mut cnt: i64 = 0;
        let mut current_node = node;
        for direction in directions.chars().cycle() {
            if current_node.ends_with('Z') {
                break;
            }
            let edges = graph.get(&current_node).unwrap();
            current_node = {
                if direction == 'L' {
                    &edges.0
                } else if direction == 'R' {
                    &edges.1
                } else {
                    panic!("tried to follow direction '{direction:?}'")
                }
            }
            .to_owned();
            cnt += 1;
        }
        answers.push(cnt);
    }
    println!(
        "{:?}",
        answers.iter().fold(1, |x, y| num::integer::lcm(x, *y))
    );
}

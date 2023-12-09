use core::panic;
use std::{collections::HashMap, io::stdin};

use regex::Regex;

fn main() {
    let mut directions = String::new();
    stdin().read_line(&mut directions).unwrap();
    let directions = directions.trim().to_owned();
    let re = Regex::new(r"([A-Z]+) = \(([A-Z]+), ([A-Z]+)\)").unwrap();
    let mut nodes = HashMap::new();
    for line in stdin().lines().skip(1) {
        let line = line.unwrap();
        let m = re.captures(&line).unwrap();
        nodes.insert(m[1].to_owned(), (m[2].to_owned(), m[3].to_owned()));
    }
    for entry in &nodes {
        println!("{} -> {},{}", entry.0, entry.1 .0, entry.1 .1);
    }
    let mut node = String::from("AAA");
    let mut cnt = 0;
    for direction in directions.chars().cycle() {
        if node == "ZZZ" {
            break;
        }
        let edges = nodes.get(&node).unwrap();
        let next = {
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
        node = next;
    }
    println!("{}", cnt);
}

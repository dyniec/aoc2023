use core::num;
use std::collections::{ HashMap,HashSet};
use std::io::stdin;
type Pos = (usize, usize);
fn neighbours((x, y): Pos, (size): Pos) -> Vec<Pos> {
    let (x, y) = (x as isize, y as isize);
    let mut res = Vec::new();
    for dx in [-1, 0, 1] {
        for dy in [-1, 0, 1] {
            if dx == 0 && dy == 0 {
                continue;
            }
            let (x1, y1) = (x + dx, y + dy);
            if x1 >= 0 && x1 < (size.0 as isize) && y1 >= 0 && y1 < (size.1 as isize) {
                res.push((x1 as usize, y1 as usize));
            }
        }
    }
    res
}
fn main() {
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in stdin().lines() {
        map.push(line.unwrap().chars().collect());
    }
    let mut numbers_per_symbol: HashMap<(usize, usize), Vec<i32>> = HashMap::new();
    let size = (map.len(), map.first().unwrap().len());
    for x in 0..size.0 {
        let mut current_number = 0;
        let mut symbols = HashSet::new();

        for y in 0..size.1 {
            let c = map[x][y];
            if let Some(d) = c.to_digit(10) {
                current_number = current_number * 10 + (d as i32);
                for (x1, y1) in neighbours((x, y), size) {
                    let c = map[x1][y1];
                    if c == '*' {
                        symbols.insert((x1, y1));
                    }
                }
            } else {
                if current_number != 0 {
                    for entry in &symbols {
                        numbers_per_symbol
                            .entry(*entry)
                            .or_default()
                            .push(current_number);
                    }
                }
                current_number = 0;
                symbols.clear();
            }
        }
        if current_number != 0 {
            for entry in symbols {
                numbers_per_symbol
                    .entry(entry)
                    .or_default()
                    .push(current_number);
            }
        }
    }
    println!("{:?}", numbers_per_symbol);
    let result = numbers_per_symbol
        .iter()
        .filter(|x| x.1.len() == 2)
        .map(|x| x.1.iter().map(|x|*x as i64).product::<i64>())
        .sum::<i64>();
    println!("{}", result);
}

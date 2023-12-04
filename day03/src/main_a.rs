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
    let size = (map.len(), map.first().unwrap().len());
    let mut sum_numbers = Vec::new();
    for x in 0..size.0 {
        let mut current_number = 0;
        let mut next_to_symbol = false;
        for y in 0..size.1 {
            let c = map[x][y];
            if let Some(d) = c.to_digit(10) {
                current_number = current_number * 10 + (d as i32);
                if !next_to_symbol {
                    for (x1, y1) in neighbours((x, y), size) {
                        let c = map[x1][y1];
                        if !c.is_digit(10) && c != '.' {
                            next_to_symbol = true;
                            break;
                        }
                    }
                }
            } else {
                if next_to_symbol {
                    sum_numbers.push(current_number);
                    next_to_symbol = false;
                }
                current_number = 0;
            }
        }
        if next_to_symbol {
            sum_numbers.push(current_number);
        }
    }
    println!("{:?}", sum_numbers);
    println!("{}", sum_numbers.iter().sum::<i32>());
}

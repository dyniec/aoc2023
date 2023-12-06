use std::io::stdin;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let time = buf
        .trim()
        .split_ascii_whitespace()
        .skip(1)
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    buf.clear();
    stdin().read_line(&mut buf).unwrap();
    let distance = buf
        .trim()
        .split_ascii_whitespace()
        .skip(1)
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    println!("{:?}", distance);
    let mut result = 1;
    for (T, D) in time.iter().zip(&distance) {
        println!("Started {} {}", T, D);
        let mut cnt = 0;
        for t in 1..*T {
            if t * (T - t) > *D {
                // print!("{} ", t);
                cnt += 1;
            }
        }
        println!("with count {}", cnt);
        result *= cnt;
    }
    println!("{}", result);
}

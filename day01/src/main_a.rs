use std::io::stdin;

fn main() {
    let mut result = 0;
    for line in stdin().lines(){
        let  line = line.unwrap();
        let digits = line.chars().filter(|x| x.is_digit(10)).map(|x|x.to_string().parse::<i32>().unwrap()).collect::<Vec<_>>();
        let first = digits.first().unwrap();
        let last = digits.last().unwrap();
        result += 10*first + last;
    }
    println!("{result}");
}

use std::io::stdin;
fn main() {

    let mut result = 0;
    for line in stdin().lines(){
        let mut line = line.unwrap();
        line=line.replace("zero", "zero0zero");
        line=line.replace("one", "one1one");
        line=line.replace("two", "two2two");
        line=line.replace("three", "three3three");
        line=line.replace("four", "four4four");
        line=line.replace("five", "five5five");
        line= line.replace("six", "six6six");
        line=line.replace("seven", "seven7seven");
        line = line.replace("eight", "eight8eight");
        line= line.replace("nine", "nine9nine");
        let digits = line.chars().filter(|x| x.is_digit(10)).map(|x|x.to_string().parse::<i32>().unwrap()).collect::<Vec<_>>();
        let first = digits.first().unwrap();
        let last = digits.last().unwrap();
        result += 10*first + last;
    }
    println!("{result}");
}

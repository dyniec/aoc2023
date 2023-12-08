use std::{io::stdin, char::from_digit, collections::HashMap};

#[derive(PartialEq,Eq,PartialOrd,Ord,Clone,Debug,Hash)]
enum Card{
    J,
    Num(u32),
    Q,
    K,
    A,
}
impl Card {
    fn parse_char(c: char)->Card{
        if let Some(n) = c.to_digit(10){
            return Card::Num(n)
        }
        return match c{
            'T' => Card::Num(10),
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => 
                panic!("Cannot parse char {c} as card")
        }
    }
}

#[derive(PartialEq,Eq,PartialOrd,Ord,Clone,Debug)]
enum HandType{
    HighCard,
    Pair,
    TwoPairs,
    Three,
    FullHouse,
    Four,
    Five,

}
impl HandType{
    fn classify(cards: &[Card])-> HandType{
        use Card::*;
        use HandType::*;
        let mut map:HashMap<&Card,i32> = HashMap::new();
        for c in cards{
            *map.entry(c).or_insert(0)+=1;
        }
        let j_count=map.get(&J).unwrap_or(&0).to_owned();
        map.remove(&J);
        if j_count ==5{
            return Five
        }
        let mut v:Vec<i32> = map.into_values().collect();
        v.sort();
        *v.last_mut().unwrap()+=j_count;
        match &v[..] {
            &[5] => Five,
            &[1,4] => Four,
            &[2,3] => FullHouse,
            &[1,1,3] => Three,
            &[1,2,2] => TwoPairs,
            &[1,1,1,2] => Pair,
            &[1,1,1,1,1] => HighCard,
            _ => panic!("Did I forget to implement to classify hand {v:?}")

        }
    }
}
#[derive(PartialEq,Eq,PartialOrd,Ord,Clone,Debug)]
struct Hand(HandType,Vec<Card>);
impl Hand {
    fn build(s: &str)-> Hand{
        let v:Vec<_> = s.chars().map(Card::parse_char).collect();
        Hand(HandType::classify(&v),v)
    }
}
fn main() {
    let mut result = Vec::new();
    for line in stdin().lines(){
        let line=line.unwrap();
        let line = line.trim().split_ascii_whitespace().collect::<Vec<_>>();
        let hand =Hand::build(line[0]);
        let bet = line[1].parse::<i64>().unwrap();
        result.push((hand,bet));
    }
    result.sort();
    let answer = result.iter().enumerate().map(|(x,(y,z))| (x as i64+1)*(*z)).sum::<i64>();
    
    println!("{:?}",result);
    for (x,y) in result{
        println!("{:?}\t\t{:?} {}",x.0, x.1,y);
    }
    println!("{answer}");
}

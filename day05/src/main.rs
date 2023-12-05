use std::io::stdin;
#[derive(PartialEq,Eq,PartialOrd,Ord,Debug,Clone)]
struct Range {
    from: u64, //inclusive
    to: u64,   //exclusive
}
impl Range {
    fn new(start:u64, len:u64) -> Self{
        Self{
            from:start,
            to:start+len,
        }
    }
    fn intersect(&self, other: &Self) -> Option<(Self, Vec<Self>)> {
        if self.to <= other.from || other.to <= self.from{
            return None
        }
        if self.from <=other.from{
            let mut outside = Vec::new();
            if self.from <other.from {
                outside.push(Range{from:self.from,to:other.from});
            }
            if self.to > other.to {
                outside.push(Range{from:other.to,to:self.to});
            }
            Some((Range{from:self.from.max(other.from),to:self.to.min(other.to)},outside))
    
        } else {

            let mut outside = Vec::new();
            if self.to > other.to {
                outside.push(Range{from:other.to,to:self.to});
            }
            Some((Range{from:self.from.max(other.from),to:self.to.min(other.to)},outside))
    
        }
    }
    fn move_base(&self, old_base:u64,new_base:u64) -> Self{

        Self{
            from:(self.from-old_base)+new_base,
            to:(self.to-old_base)+new_base,
        }
    }
}

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let seeds = buf
        .trim()
        .split(' ')
        .skip(1)
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let seeds = seeds.chunks(2).map(|x| Range::new(x[0],x[1])).collect::<Vec<_>>();

    let mut maps = Vec::new();
    {
        let mut name = String::new();
        let mut mapping = Vec::new();
        for line in stdin().lines().skip(1) {
            let line = line.unwrap();
            if line.is_empty() {
                maps.push((
                    std::mem::replace(&mut name, String::new()),
                    std::mem::replace(&mut mapping, Vec::new()),
                ));
            } else if line.ends_with(" map:") {
                name = line.replace(" map:", "");
            } else {
                let line = line
                    .trim()
                    .split(' ')
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect::<Vec<_>>();
                mapping.push((line[0], line[1], line[2]));
            }
        }
    }
    let mut locations = seeds;
    for map in &maps {
        println!("{:?}",map);
        let mut result = Vec::new();
        while let Some(mut pos) = locations.pop() {
            for (target_start, source_start, len) in &map.1 {
                match pos.intersect(&Range::new(*source_start,*len)) {
                    Some((inside,mut rest)) => {
                        pos=inside.move_base(*source_start,*target_start);
                        locations.append(&mut rest);
                        break;
                    },
                    None => {},
                }
            }
            result.push(pos)
        }
        locations=result;
    }
    println!("{}", locations.iter().min().unwrap().from);
}

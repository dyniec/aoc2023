use std::io::stdin;
fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let seeds = buf
        .trim()
        .split(' ')
        .skip(1)
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
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
    println!("{:?}", maps);
    let mut locations = Vec::new();
    for seed in &seeds{
        let mut seed = *seed;
        print!("{seed}");
        for map in &maps{
            for (target_start, source_start,len) in &map.1{
                if seed >= *source_start && seed < (*source_start+len){
                    seed = (seed-*source_start)+ *target_start;
                    break;
                }
            }
            print!("-> {seed} ");
        }
        println!("");
        locations.push(seed)
    }
    println!("{:?}", locations);
    println!("{}",locations.iter().min().unwrap());
}

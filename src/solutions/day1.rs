use std::fs;

pub fn day1() {
    let data = fs::read_to_string("src/inputs/day1input.txt").expect("Unable to read file");
    let split = data.split("\n\n");
    let vec = split.collect::<Vec<&str>>();
    let mut sumvec = Vec::new();
    for s in vec {
        let newvec = s.split("\n").collect::<Vec<&str>>();
        let mut tot = 0;
        for i in newvec{
            tot+= i.parse::<i32>().unwrap();
        }
        sumvec.push(tot);
    }
    sumvec.sort();
    println!("Part 1: {:?}", sumvec[sumvec.len()-1]);
    println!("Part 2: {:?}", sumvec[sumvec.len()-1] + sumvec[sumvec.len()-2] + sumvec[sumvec.len()-3]);
}
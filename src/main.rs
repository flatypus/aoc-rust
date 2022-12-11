use std::env;

mod solutions{
    pub mod day1;
}

use crate::solutions::day1::day1;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1{
        if args[1] == "1"{
            println!("Day {}", args[1]);
            day1()
        } else{
            println!("Day 1");
            day1()
        }
    } else{
        println!("Day 1");
        day1()
    }
}

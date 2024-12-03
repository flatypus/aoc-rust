use lazy_static::lazy_static;

lazy_static! {
    static ref RE: regex::Regex = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
}

fn sum(line: &str) -> i32 {
    let mut count = 0;
    for caps in RE.captures_iter(line) {
        let a: i32 = caps[1].parse().unwrap();
        let b: i32 = caps[2].parse().unwrap();
        count += a * b;
    }
    count
}
#[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {
    sum(input)
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> i32 {
    let mut part_2 = 0;
    for do_lines in input.split("do()") {
        if let Some(valid) = do_lines.split("don't()").next() {
            part_2 += sum(valid);
        }
    }
    part_2
}

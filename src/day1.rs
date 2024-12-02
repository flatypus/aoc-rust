use std::collections::HashMap;

fn split_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|line| {
            let mut nums = line.split_whitespace().map(|x| x.parse::<i32>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .unzip()
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let (mut left, mut right) = split_input(input);
    left.sort_unstable();
    right.sort_unstable();
    let sum: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs() as i32)
        .sum();
    sum
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let (left, right) = split_input(input);
    let mut count: HashMap<&i32, i32> = HashMap::new();
    for i in right.iter() {
        count.entry(i).or_insert(0);
    }
    let mut sum: i32 = 0;
    for l in left.iter() {
        sum += l * count.get(l).unwrap_or(&0);
    }
    sum
}

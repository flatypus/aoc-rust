use gxhash::{HashMap, HashMapExt};

fn split(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left = Vec::with_capacity(1000);
    let mut right = Vec::with_capacity(1000);

    let i: i16 = 0;
    for num in input.split_ascii_whitespace() {
        let num = num.parse::<i32>().unwrap();
        if i % 2 == 0 {
            left.push(num);
        } else {
            right.push(num);
        }
    }
    (left, right)
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let (mut left, mut right) = split(input);
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
    let (left, right) = split(input);
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

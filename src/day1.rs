fn split_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let (left, right): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            let mut nums = line.split_whitespace().map(|x| x.parse::<i32>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .unzip();
    (left, right)
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let (mut left, mut right) = split_input(input);
    left.sort();
    right.sort();
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
    let sum: i32 = left
        .iter()
        .map(|l| {
            right
                .iter()
                // count number of times l appears in right
                .filter(|r| l == *r)
                .count() as i32
        })
        .sum();
    sum
}

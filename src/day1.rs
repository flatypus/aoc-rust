use std::collections::HashMap;

fn split(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left = Vec::with_capacity(1000);
    let mut right = Vec::with_capacity(1000);

    let mut i: i16 = 0;
    for num in input.split_ascii_whitespace() {
        let num = num.parse::<i32>().unwrap();
        if i % 2 == 0 {
            left.push(num);
        } else {
            right.push(num);
        }
        i += 1;
    }

    (left, right)
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let (mut left, mut right) = split(input);
    left.sort_unstable();
    right.sort_unstable();
    let mut sum: i32 = 0;
    for i in 0..left.len() {
        sum += (left[i] - right[i]).abs();
    }
    sum
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let (left, right) = split(input);
    let mut count: HashMap<&i32, i32> = HashMap::new();
    for i in right.iter() {
        count.entry(i).and_modify(|e| *e += 1).or_insert_with(|| 1);
    }
    let mut sum: i32 = 0;
    for l in left.iter() {
        sum += l * count.get(l).unwrap_or(&0);
    }
    sum
}

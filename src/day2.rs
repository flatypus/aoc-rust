fn is_valid_sequence(line: &Vec<i32>) -> bool {
    let mut diff = line[1] - line[0];
    if !(1 <= diff.abs() && diff.abs() <= 3) {
        return false;
    }

    let direction = diff / diff.abs();
    for i in 1..line.len() {
        diff = line[i] - line[i - 1];
        if !(1 <= diff.abs() && diff.abs() <= 3) || diff / diff.abs() != direction {
            return false;
        }
    }
    true
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> i32 {
    let mut count = 0;
    for line in input.split('\n') {
        let mut row = Vec::with_capacity(8);
        for x in line.split_whitespace() {
            row.push(x.parse::<i32>().unwrap());
        }
        if is_valid_sequence(&row) {
            count += 1;
        }
    }
    count
}

fn is_part_2_valid_sequence(line: &Vec<i32>) -> bool {
    if is_valid_sequence(line) {
        return true;
    }

    if line.len() == 2 {
        return false;
    }

    for i in 0..line.len() {
        let mut new_v = Vec::with_capacity(7);
        for j in 0..line.len() {
            if i == j {
                continue;
            }
            new_v.push(line[j]);
        }
        if is_valid_sequence(&new_v) {
            return true;
        }
    }
    false
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> i32 {
    let mut count = 0;
    for line in input.split('\n') {
        let mut row = Vec::with_capacity(8);
        for x in line.split_whitespace() {
            row.push(x.parse::<i32>().unwrap());
        }
        if is_part_2_valid_sequence(&row) {
            count += 1;
        }
    }
    count
}

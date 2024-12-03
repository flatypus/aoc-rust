fn is_valid_sequence(line: &[i32]) -> bool {
    let len = line.len();
    if len < 2 {
        return false;
    }

    let diff = line[1] - line[0];
    if diff.abs() < 1 || diff.abs() > 3 {
        return false;
    }

    let direction = diff.signum();
    for i in 1..len {
        let diff = line[i] - line[i - 1];
        if diff.abs() < 1 || diff.abs() > 3 || diff.signum() != direction {
            return false;
        }
    }
    true
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> i32 {
    input
        .lines()
        .filter_map(|line| {
            let row: Vec<i32> = line
                .split_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect();
            if is_valid_sequence(&row) {
                Some(1)
            } else {
                None
            }
        })
        .count() as i32
}

fn is_part_2_valid_sequence(line: &[i32]) -> bool {
    if is_valid_sequence(line) {
        return true;
    }

    if line.len() <= 2 {
        return false;
    }

    for i in 0..line.len() {
        // Simulate removal without creating a new vector
        if is_valid_sequence(
            &line[..i]
                .iter()
                .chain(&line[i + 1..])
                .cloned()
                .collect::<Vec<_>>(),
        ) {
            return true;
        }
    }
    false
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> i32 {
    input
        .lines()
        .filter_map(|line| {
            let row: Vec<i32> = line
                .split_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect();
            if is_part_2_valid_sequence(&row) {
                Some(1)
            } else {
                None
            }
        })
        .count() as i32
}

use std::fs;

fn main() {
    let input = "3   4
4   3
2   5
1   3
3   9
3   3";

    assert_eq!(11i64, day1_part1(input));
    let result = day1_part1(fs::read_to_string("./src/bin/day1/input.txt").unwrap());
    println!("Day 1 part1: {result}");

    assert_eq!(31i64, day1_part2(input));
    let result = day1_part2(fs::read_to_string("./src/bin/day1/input.txt").unwrap());
    println!("Day 1 part2: {result}");
}

fn day1_part1(input: impl AsRef<str>) -> i64 {
    let (lefts, rights) = parse_input(input);

    let result: i64 = lefts.iter().zip(rights)
        .map(|(left, right)| (left - right).abs())
        .sum();
    result
}

fn day1_part2(input: impl AsRef<str>) -> i64 {
    let (lefts, rights) = parse_input(input);
    let mut similarity_score: i64 = 0;
    let mut rights = rights.into_iter().peekable();
    let mut last = (0,0);

    for left in lefts {
        if left == last.0 {
            similarity_score += last.1;
        } else {
            let mut appearances_on_right = 0;
            loop {
                let right = rights.peek().cloned();
                if let Some(right) = right {
                    if right <= left {
                        rights.next();
                        if right == left {
                            appearances_on_right += 1;
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            similarity_score += left * appearances_on_right;
            last = (left, left * appearances_on_right);
        }
    }

    similarity_score
}

fn parse_input(input: impl AsRef<str> + Sized) -> (Vec<i64>, Vec<i64>) {
    let input = input.as_ref();
    let mut lefts = vec![];
    let mut rights = vec![];

    for line in input.lines() {
        let numbers = line
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .take(2)
            .collect::<Vec<i64>>();
        lefts.push(*numbers.first().unwrap());
        rights.push(*numbers.get(1).unwrap());
    }

    lefts.sort();
    rights.sort();

    (lefts, rights)
}
use std::fs;

fn main() {
    assert_eq!(11i64, day1_part1("3   4
4   3
2   5
1   3
3   9
3   3"));
    let result = day1_part1(fs::read_to_string("./src/bin/day1/input.txt").unwrap());
    println!("Day 1 part1: {result}");
}

fn day1_part1(input: impl AsRef<str>) -> i64 {
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
    let result: i64 = lefts.iter().zip(rights)
        .map(|(left, right)| (left - right).abs())
        .sum();
    result
}

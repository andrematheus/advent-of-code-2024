use std::fs;
use regex::Regex;

fn main() {
    let example = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    let example2 = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    let input = fs::read_to_string("./src/bin/day3/input.txt").unwrap();

    assert_eq!(161, day2_part1(example));
    let result = day2_part1(&input);
    println!("Day 2 part1: {result}");

    assert_eq!(48, day2_part2(example2));
    let result = day2_part2(&input);
    println!("Day 2 part2: {result}");
}

fn day2_part1(input: impl AsRef<str>) -> i64 {
    let regex = Regex::new("mul\\(([0-9]+),([0-9]+)\\)").unwrap();
    regex.captures_iter(input.as_ref()).map(|m| {

        let x = m.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let y = m.get(2).unwrap().as_str().parse::<i64>().unwrap();
        x * y
    }).sum()
}

fn day2_part2(input: impl AsRef<str>) -> i64 {
    let mut enabled = true;

    let regex = Regex::new("((do\\(\\)|don't\\(\\))|(mul\\(([0-9]+),([0-9]+)\\)))").unwrap();
    regex.captures_iter(input.as_ref()).map(|m| {
        if m.get(2).is_some() {
            let instruction = m.get(2).unwrap().as_str();
            if instruction == "do()" {
                enabled = true;
            } else if instruction == "don't()" {
                enabled = false
            } else {
                panic!("Invalid instruction {}", instruction);
            }
            0
        } else if enabled {
            let x = m.get(4).unwrap().as_str().parse::<i64>().unwrap();
            let y = m.get(5).unwrap().as_str().parse::<i64>().unwrap();
            x * y
        } else {
            0
        }
    }).sum()
}
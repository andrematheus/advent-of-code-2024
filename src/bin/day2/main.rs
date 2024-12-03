use std::fs;

fn main() {
    let example = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    let input = fs::read_to_string("./src/bin/day2/input.txt").unwrap();

    assert_eq!(2, day2_part1(example));
    let result = day2_part1(&input);
    println!("Day 2 part1: {result}");

    assert_eq!(4, day2_part2(example));
    let result = day2_part2(&input);
    println!("Day 2 part2: {result}");
}

fn day2_part1(input: impl AsRef<str>) -> usize {
    let input = parse_input(input);
    input.iter()
        .map(|level| level.is_safe())
        .filter(|b| *b)
        .count()
}

fn day2_part2(input: impl AsRef<str>) -> usize {
    let input = parse_input(input);
    input.iter()
        .map(|mut level| level.safe_after_dampening())
        .filter(|b| *b)
        .count()
}

fn parse_input(input: impl AsRef<str>) -> Vec<Vec<i64>> {
    input.as_ref().lines()
        .map(|l| l.split_whitespace().collect())
        .map(|toks: Vec<&str>| toks.iter()
            .map(|s| s.parse::<i64>().unwrap())
            .collect()
        ).collect()
}

trait Level {
    fn is_safe(&self) -> bool;
    fn is_increasing_or_decreasing(&self) -> bool;
    fn adjacents_differ_at_least_at_most(&self, at_least: i64, at_most: i64) -> bool;
    fn safe_after_dampening(&mut self) -> bool;
}

impl Level for &Vec<i64> {
    fn is_safe(&self) -> bool {
        self.is_increasing_or_decreasing() && self.adjacents_differ_at_least_at_most(1, 3)
    }

    fn is_increasing_or_decreasing(&self) -> bool {
        let is_increasing = self.get(1) > self.get(0);
        for i in 1..self.len() {
            if ((self[i] < self[i - 1]) && is_increasing) || (self[i] > self[i - 1]) && !is_increasing {
                return false;
            }
        }
        true
    }

    fn adjacents_differ_at_least_at_most(&self, at_least: i64, at_most: i64) -> bool {
        for i in 1..self.len() {
            let diff = (self[i] - self[i - 1]).abs();
            if diff < at_least || diff > at_most {
                return false;
            }
        }
        true
    }

    fn safe_after_dampening(&mut self) -> bool {
        if self.is_safe() {
            true
        } else {
            for i in 0..self.len() {
                let mut v = self.clone();
                v.remove(i);
                if (&v).is_safe() {
                    return true;
                }
            }
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe() {
        let v: &Vec<i64> = &vec![7,6,4,2,1];
        assert!(v.is_safe());
    }

    #[test]
    fn test_increasing_or_decreasing() {
        let v: &Vec<i64> = &vec![2,1,4];
        assert!(!v.is_increasing_or_decreasing());
    }
}
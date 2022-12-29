use std::{time::{Instant, Duration}, cmp::Ordering, fmt};

use crate::util::{get_filename, read_lines};

const DEBUG: bool = false;
// Every section in the camp has a unique ID
// Every elf is assigned to a range of sections
// However, some section assignments overlap
// Part 1: How many pairs have one range entirely within the other range?

// Steps
// Get lines y
// Look at a line & split it into two ranges
// Parse a range to get the top & bottom numbers
// If (bottom A < bottom B && top A > top B) || (bottom B < bottom A && top B > top A)

fn split_ranges(line: String) -> (String, String) {
    let mut split = line.split(",");
    let res1 = split.next().unwrap();
    let res2 = split.next().unwrap();
    (res1.to_string(), res2.to_string())
}
struct Range {
    lower: isize,
    upper: isize
}

impl Range {
    fn create(input: String) -> Range {
        let mut split = input.split("-");
        let lower = split.next().unwrap().to_string().parse().unwrap();
        let upper = split.next().unwrap().to_string().parse().unwrap();
        Range{
            lower: lower,
            upper: upper
        }
    }

    fn cmp(&self, other: &Range) -> Ordering {
        if self.lower < other.lower {
            return Ordering::Less;
        } else if self.lower > other.lower {
            return Ordering::Greater;
        } else {
            return Ordering::Equal;
        }
    }

    fn contains(&self, other: &Range) -> bool {
        self.lower <= other.lower && self.upper >= other.upper
    }
}

fn complete_overlap(range1: &Range, range2: &Range) -> bool {
    let contains: bool;
    match range1.cmp(&range2) {
        Ordering::Less => {
            contains = range1.contains(&range2);
            if DEBUG && !contains{
                println!("      {} contains {}: {}", &range1, &range2, contains)
            }
        }
        Ordering::Greater => {
            contains = range2.contains(&range1);
            if DEBUG && !contains {
                println!("      {} contains {}: {}", &range2, &range1, contains)
            }
        }
        Ordering::Equal => {
            contains = range2.contains(&range1) || range1.contains(&range2);
        }
    }
    contains
}

fn any_overlap(range1: &Range, range2: &Range) -> bool {
    let contains: bool;
    if range1.lower == range2.lower {
        contains = true;
    } else if range1.lower < range2.lower {
        contains = range1.upper >= range2.lower
    } else {
        contains = range2.upper >= range1.lower
    }
    if DEBUG {
        print!("    Comparing {} with {}. ", &range1, &range2);
        match contains {
            true => println!("Overlap"),
            false => println!("No overlap"),
        }
    }
    contains
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.lower, self.upper)
    }
}

pub fn part1(input: super::Data) -> (usize, Duration) {
    let now = Instant::now();
    let mut res: usize = 0;

    // Get lines
    let lines = read_lines(get_filename("day4", input));
    if let Ok(lines) = lines {
        for line in lines {
            if let Ok(line) = line {
                let (range1, range2) = split_ranges(line);
                let range1 = Range::create(range1);
                let range2 = Range::create(range2);
                if DEBUG {
                    println!("    Comparing {} with {}.", &range1, &range2);
                }
                if complete_overlap(&range1, &range2) {
                    res += 1;
                }
            }
        }

    }

    (res, now.elapsed())
}

pub fn part2(input: super::Data) -> (usize, Duration) {
    let now = Instant::now();
    let mut res: usize = 0;
    let lines = read_lines(get_filename("day4", input));
    if let Ok(lines) = lines {
        for line in lines {
            if let Ok(line) = line {
                let (range1, range2) = split_ranges(line);
                let range1 = Range::create(range1);
                let range2 = Range::create(range2);
                if any_overlap(&range1, &range2) {
                    res += 1;
                }
            }
        }

    }

    (res, now.elapsed())
}

#[cfg(test)]
#[macro_use]
mod tests {
    use table_test::table_test;

    use crate::Data;
    #[test]
    fn part1() {
        let (res, _dur) = super::part1(Data::Test);
        let want = 2;
        assert_eq!(want, res);
    }

    #[test]
    fn test_split_ranges() {
        let table = vec![
            ("2-4,6-8", ("2-4", "6-8")),
            ("2-3,4-5", ("2-3", "4-5")),
            ("5-7,7-9", ("5-7", "7-9"))
        ];

        for (validator, input, (expected1, expected2)) in table_test!(table) {
            let actual = super::split_ranges(input.to_string());

            validator
                .given(&format!("{}", input.to_string()))
                .when("split_ranges")
                .then(&format!("it should be ({}, {})", expected1, expected2))
                .assert_eq((expected1.to_string(), expected2.to_string()), actual);
        }
    }

    #[test]
    fn part2() {
        let (res, _dur) = super::part2(Data::Test);
        let want = 4;
        assert_eq!(want, res);
    }
}

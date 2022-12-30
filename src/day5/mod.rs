const DEBUG: bool = false;
// use super::Data;
use std::time::{Instant, Duration}; 
use crate::util::{get_filename, read_lines};
use std::io::{self, BufReader};
use std::fs::File;
use regex::Regex;

#[derive(Debug, PartialEq)]
struct Hanoi {
    stacks: Vec<Vec<char>>,
    instructions: Vec<String>
}

impl Hanoi {
    fn create(mut file: io::Lines<BufReader<File>>) -> Hanoi {
        let mut lines = Vec::new();
        let mut hanoi = Hanoi{
            stacks: Vec::new(),
            instructions: Vec::new()
        };
        loop {
            let line = file.next().unwrap().unwrap();
            if line == "" {
                break;
            } else {
                lines.push(line);
            }
        }
        for _ in 0..Hanoi::no_stacks(lines.pop().unwrap()) {
            hanoi.stacks.push(Vec::new());
        }
        loop {
            let line = lines.pop();
            if let None = line {
                break;
            }
            hanoi.parse_row(line.unwrap());
        }
        loop {
            let line = file.next();
            if let Some(line) = line {
                if let Ok(line) = line {
                    hanoi.instructions.push(line);
                }
            } else {
                break;
            }
        }
        hanoi
    }

    fn get_tops(&self) -> String {
        let mut res: String = String::new();
        for n in 0..self.stacks.len() {
            if self.stacks[n].len() == 0 {
                res.push(' ');
            } else {
                res.push(self.stacks[n].last().unwrap().clone());
            }
        }

        res
    }

    fn parse_row(&mut self, row: String) {
        let mut chars = row.chars();
        let mut stack: usize = 0;
        let mut curr_char: char;
        // Get first character
        let next_char = chars.next();
        if let None = next_char {
            return;
        }
        loop {
            let next_char = chars.next();
            if let None = next_char {
                return;
            }
            curr_char = next_char.unwrap();
            if curr_char != ' ' {
                self.stacks[stack].push(curr_char);
            }
            stack += 1;
            if stack >= self.stacks.len() {
                return;
            }
            for _ in 0..3 {
                let chomp = chars.next();
                if let None = chomp {
                    return;
                }
            }
        }
    }

    fn move_stack(&mut self, no: usize, curr: usize, dest: usize) {
        if DEBUG {
            println!("Moving {} items from stack {} to stack {}.", no, curr+1, dest+1);
        }
        let mut move_stack = Vec::new();
        for _ in 0..no {
            move_stack.push(self.stacks[curr].pop().unwrap());
        }
        if DEBUG {
            println!("Stack to move (reversed): {:?}", &move_stack);
        }
        move_stack.reverse();
        for _ in 0..move_stack.len() {
            self.stacks[dest].push(move_stack.pop().unwrap());
        }
    }

    fn move_stack_multiple(&mut self, no: usize, curr: usize, dest: usize) {
        if DEBUG {
            println!("Moving {} items from stack {} to stack {}.", no, curr+1, dest+1);
        }
        let mut move_stack = Vec::new();
        for _ in 0..no {
            move_stack.push(self.stacks[curr].pop().unwrap());
        }
        if DEBUG{
            println!("Stack to move: {:?}", &move_stack);
        }
        for _ in 0..move_stack.len() {
            self.stacks[dest].push(move_stack.pop().unwrap());
        }
    }

    fn no_stacks(stackline: String) -> isize {
        let mut res: isize = 0;
        for letter in stackline.chars() {
            if letter != ' ' {
                res += 1;
            }
        }
        res
    }

    fn parse_instruction(instruction: String) -> (usize, usize, usize) {
        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let caps = re.captures(instruction.as_str()).unwrap();
        (
            caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            caps.get(2).unwrap().as_str().parse::<usize>().unwrap()-1,
            caps.get(3).unwrap().as_str().parse::<usize>().unwrap()-1
        )
    }
}

pub fn part1(input: crate::Data) -> (String, Duration) {
    let now = Instant::now();
    let lines = read_lines(get_filename("day5", input)).unwrap();
    let mut hanoi = Hanoi::create(lines);
    for instruction in hanoi.instructions.clone(){
        let (no, curr, dest) = Hanoi::parse_instruction(instruction);
        hanoi.move_stack(no, curr, dest);
    }

    (hanoi.get_tops(), now.elapsed())
}

pub fn part2(input: crate::Data) -> (String, Duration) {
    let now = Instant::now();
    let lines = read_lines(get_filename("day5", input)).unwrap();
    let mut hanoi = Hanoi::create(lines);
    for instruction in hanoi.instructions.clone(){
        let (no, curr, dest) = Hanoi::parse_instruction(instruction);
        hanoi.move_stack_multiple(no, curr, dest);
    }

    (hanoi.get_tops(), now.elapsed())
}

#[cfg(test)]
#[macro_use]
mod tests {
    use table_test::table_test;
    use super::*;
    use crate::Data;

    #[test]
    fn test_get_no_stacks() {
        let table: Vec<(&str, isize)> = vec![
            ("1 2 3", 3),
            (" 1   2   3   4   5   6   7   8   9 ", 9),
        ];

        for (validator, input, expected) in table_test![table] {
            let actual = Hanoi::no_stacks(input.to_string());

            validator
                .given(&format!("{}", input))
                .when("no_stacks")
                .then(&format!("it should be {}", expected))
                .assert_eq(expected, actual);
        }
    }

    #[test]
    fn create_hanoi() {
        let lines = read_lines(get_filename("day5", Data::Test)).unwrap();
        let actual = Hanoi::create(lines);
        let expected = Hanoi {
            stacks: vec![
                vec!['Z', 'N'],
                vec!['M', 'C', 'D'],
                vec!['P']
            ],
            instructions: vec![
                String::from("move 1 from 2 to 1"),
                String::from("move 3 from 1 to 3"),
                String::from("move 2 from 2 to 1"),
                String::from("move 1 from 1 to 2")
            ]
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn move_stack() {
        let lines = read_lines(get_filename("day5", Data::Test)).unwrap();
        let mut hanoi = Hanoi::create(lines);
        let mut expected = Hanoi {
            stacks: vec![
                vec!['Z', 'N'],
                vec!['M', 'C', 'D'],
                vec!['P']
            ],
            instructions: vec![
                String::from("move 1 from 2 to 1"),
                String::from("move 3 from 1 to 3"),
                String::from("move 2 from 2 to 1"),
                String::from("move 1 from 1 to 2")
            ]
        };
        assert_eq!(expected, hanoi);
        hanoi.move_stack(1, 1, 0);
        expected.stacks[0] = vec!['Z', 'N', 'D'];
        expected.stacks[1] = vec!['M', 'C'];
        assert_eq!(expected, hanoi);

        hanoi.move_stack(3, 0, 2);
        expected.stacks[0] = Vec::new();
        expected.stacks[2] = vec!['P', 'D', 'N', 'Z'];
        assert_eq!(expected, hanoi);
    }

    #[test]
    fn parse_instruction() {
        let table = vec![
            ("move 1 from 2 to 1", (1, 1, 0)),
            ("move 3 from 1 to 3", (3, 0, 2)),
            ("move 2 from 2 to 1", (2, 1, 0)),
            ("move 1 from 1 to 2", (1, 0, 1))
        ];

        for (validator, input, (expected1, expected2, expected3)) in table_test!(table) {
            let (actual1, actual2, actual3) = Hanoi::parse_instruction(input.to_string()); 

            validator
                .given(&format!("{}", input))
                .when("Hanoi::create")
                .then(&format!("should be ({}, {}, {})", expected1, expected2, expected3))
                .assert_eq(expected1, actual1)
                .assert_eq(expected2, actual2)
                .assert_eq(expected3, actual3);
        }
    }

    #[test]
    fn part1() {
        let (res, _dur) = super::part1(Data::Test);
        let expected = String::from("CMZ");

        assert_eq!(expected, res);
    }
}
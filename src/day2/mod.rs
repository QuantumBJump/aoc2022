use crate::util::{read_lines, get_filename};
use std::time::{Instant, Duration};

use super::Data;

// Score for a single round:
// Score for shape - Rock, Paper, Scissors = 1, 2, 3
// Score for outcome - Loss, Draw, Win = 0, 3, 6

// Guide:
// Opponent - A, B, C for RPS
// You - XYZ for RPS

#[derive(Debug, PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors
}

enum Result {
    Loss,
    Draw,
    Win
}

fn letter_to_hand(letter: &str) -> Hand {
    match letter {
        "A" | "X" => Hand::Rock,
        "B" | "Y" => Hand::Paper,
        _ => Hand::Scissors
    }
}

#[derive(Debug, PartialEq)]
struct Round {
    you: Hand,
    opponent: Hand
}
impl Round {
    fn cheat(op: Hand, res: Result) -> Round{
        match op {
            Hand::Rock => {
                match res {
                    Result::Win => Round{
                        opponent: op,
                        you: Hand::Paper
                    },
                    Result::Loss => Round{
                        opponent: op,
                        you: Hand::Scissors
                    },
                    Result::Draw => Round{
                        opponent: op,
                        you: Hand::Rock
                    }
                }
            }
            Hand::Paper => {
                match res {
                    Result::Win => Round{
                        opponent: op,
                        you: Hand::Scissors
                    },
                    Result::Loss => Round{
                        opponent: op,
                        you: Hand::Rock
                    },
                    Result::Draw =>Round{
                        opponent: op,
                        you: Hand::Paper
                    }
                }
            }
            Hand::Scissors => {
                match res {
                    Result::Win => Round{
                        opponent: op,
                        you: Hand::Rock
                    },
                    Result::Loss => Round{
                        opponent: op,
                        you: Hand::Paper
                    },
                    Result::Draw => Round{
                        opponent: op,
                        you: Hand::Scissors
                    }
                }
            }
        }
    }

    fn result(&self) -> Result {
        match self.you {
            Hand::Rock => {
                match self.opponent {
                    Hand::Rock => Result::Draw,
                    Hand::Paper => Result::Loss,
                    Hand::Scissors => Result::Win
                }
            }
            Hand::Paper => {
                match self.opponent {
                    Hand::Rock => Result::Win,
                    Hand::Paper => Result::Draw,
                    Hand::Scissors => Result::Loss
                }
            }
            Hand::Scissors => {
                match self.opponent {
                    Hand::Rock => Result::Loss,
                    Hand::Paper => Result::Win,
                    Hand::Scissors => Result::Draw
                }
            }
        }

    }

    fn shape(&self) -> isize {
        match self.you {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3
        }
    }

    fn score(&self) -> isize {
        let mut res = 0;
        res += self.shape();
        match self.result() {
            Result::Win => { res += 6;}
            Result::Loss => {res += 0;}
            Result::Draw => {res += 3;}
        }
        res
    }
}

fn parse_line(line: String) -> Round {
    let instructions: Vec<&str> = line.split(" ").collect();
    Round {
        you: letter_to_hand(instructions[1]),
        opponent: letter_to_hand(instructions[0])
    }
}

fn parse_line_part2(line: String) -> isize {
    let instructions: Vec<&str> = line.split(" ").collect();
    let op: Hand;
    match instructions[0]{
        "A" => {op = Hand::Rock;}
        "B" => {op = Hand::Paper;}
        "C" => {op = Hand::Scissors;}
        _ => {op = Hand::Rock;}
    }
    let goal: Result;
    match instructions[1] {
        "X" => {goal = Result::Loss;}
        "Y" => {goal = Result::Draw;}
        "Z" => {goal = Result::Win;}
        _ => {goal = Result::Loss;}
    }

    Round::cheat(op, goal).score()
}

pub fn part1(input: Data) -> (isize, Duration) {
    let now = Instant::now();
    let file = read_lines(get_filename("day2", input));
    let mut score = 0;
    if let Ok(lines) = file {
        for line in lines {
            if let Ok(line) = line {
                let r = parse_line(line);
                score += r.score();
            }
        }
    }
    (score, now.elapsed())
}

pub fn part2(input: Data) -> (isize, Duration) {
    let now = Instant::now();
    let file = read_lines(get_filename("day2", input));
    let mut score = 0;
    if let Ok(lines) = file {
        for line in lines {
            if let Ok(line) = line {
                score += parse_line_part2(line);
            }
        }
    }
    (score, now.elapsed())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let (res, _dur) = super::part1(Data::Test);
        let want = 15;
        assert_eq!(want, res);
    }

    #[test]
    fn parse_line_ay() {
        let line = "A Y".to_string();
        let res = super::parse_line(line);
        let want = Round{
            opponent: Hand::Rock,
            you: Hand::Paper};
        assert_eq!(want, res);
    }

    #[test]
    fn parse_line_bx() {
        let line = "B X".to_string();
        let res = super::parse_line(line);
        let want = Round{
            you: Hand::Rock,
            opponent: Hand::Paper
        };
        assert_eq!(want, res);
    }
}
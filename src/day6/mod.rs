use std::time::{Instant, Duration};
use std::{fmt, fs};
use std::error;

const DEBUG: bool = false;

#[derive(Debug)]
pub enum Day6Error {
    IO(std::io::Error),
}

impl fmt::Display for Day6Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Day6Error::IO(..) => write!(f, "there was an IO error"),
        }
    }
}

impl error::Error for Day6Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Day6Error::IO(ref e) => Some(e),
        }
    }
}

impl From<std::io::Error> for Day6Error {
    fn from(err: std::io::Error) -> Day6Error {
        Day6Error::IO(err)
    }
}

use crate::util::get_filename;

struct Tape {
    length: usize,
    content: Vec<char>,
    idx: usize,
}

impl Tape {
    fn chomp(&mut self, new_char: char) {
        let curr_len = self.current_length();
        if curr_len < self.length {
            self.content.push(new_char);
        } else {
            for n in 0..self.current_length()-1 {
                self.content[n] = self.content[n+1];
            }
            self.content[curr_len-1] = new_char;
        }
        self.idx += 1;
    }

    fn current_length(&self) -> usize {
        self.content.len()
    }

    fn repeated_char(&self) -> bool {
        let mut existing  = Vec::new();
        for char in &self.content {
            if existing.contains(char) {
                if DEBUG {
                    println!("  {} repeated", char);
                }
                return true;
            }
            existing.push(*char);
        }

        false
    }
}

impl fmt::Display for Tape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "length {}: {}. idx - {}", self.current_length(), self.content.iter().collect::<String>(), self.idx)
    }
}

// Part 1 - Single row input, random-seeming letters.
// Take input, find index of first character of actual message
// Message starts after 4 consecutive different characters
pub fn part1(input: crate::Data) -> Result<(usize, Duration), Day6Error> {
    let now = Instant::now();
    let filename = get_filename("day6", input);
    let message = fs::read_to_string(filename).unwrap();
    let mut message = message.chars();

    let mut tape = Tape{
        length: 4,
        idx: 0,
        content: Vec::new(),
    };

    for _ in 0..4 {
        tape.chomp(message.next().unwrap());
    }
    if DEBUG {
        println!("{}", tape);
    }

    for char in message {
        tape.chomp(char);
        if DEBUG{
            println!("{}", tape);
        }
        if !tape.repeated_char() {
            break;
        }
    }

    Ok((tape.idx, now.elapsed()))
}

pub fn part2(input: crate::Data) -> Result<(usize, Duration), Day6Error> {
    let now = Instant::now();
    let filename = get_filename("day6", input);
    let message = fs::read_to_string(filename).unwrap();
    let mut message = message.chars();
    let mut tape = Tape{
        length: 14,
        idx: 0,
        content: Vec::new(),
    };

    // Consume first 14 chars
    for _ in 0..14 {
        tape.chomp(message.next().unwrap());
    }
    if DEBUG {
        println!("{}", tape);
    }

    for char in message {
        tape.chomp(char);
        if DEBUG{
            println!("{}", tape);
        }
        if !tape.repeated_char() {
            break;
        }
    }

    Ok((tape.idx, now.elapsed()))
}
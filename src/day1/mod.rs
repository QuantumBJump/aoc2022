// Elves go on foot
// taking inventory of supplies - puzzle input is number of calories each elf is carrying
// one item per line, \n separated. Number is amount of calories in item.
// blank line == new elf
// Goal - how many calories are being carried by the elf with the MOST calories?

// Steps
// read input file line by line

use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn lines_to_elves(lines: io::Result<io::Lines<io::BufReader<File>>>) -> Vec<Vec<String>> {
    let mut elves: Vec<Vec<String>> = Vec::new();
    let mut elf: Vec<String> = Vec::new();
    if let Ok(lines) = lines {
        for line in lines {
            if let Ok(item) = line {
                if item == "" {
                    elves.push(elf);
                    elf = Vec::new();
                } else {
                    elf.push(item);
                }
            }
        }
    }
    elves.push(elf);
    elves
}

fn elf_cals(elf: Vec<String>) -> isize {
    let mut cals: isize = 0;
    for item in elf {
        let no_cals = item.parse::<isize>().unwrap();
        cals += no_cals;
    }
    cals
}

pub fn part1(filename: &str) -> isize {
    let mut max: isize = 0;
    let file;
    if filename == "input" {
        file = "./src/day1/input.txt"
    } else {
        file = "./src/day1/test.txt"
    }
    let lines = read_lines(file);
    for elf in lines_to_elves(lines) {
        let cals = elf_cals(elf);
        if cals > max {
            max = cals;
        }
    }
    max
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let res = part1("test");
        let want = 24000;
        assert_eq!(res, want);
    }

    #[test]
    fn lines_to_elves() {
        let lines = read_lines("./src/day1/test.txt");
        let res = super::lines_to_elves(lines);
        let want = vec![
            vec!["1000", "2000", "3000"],
            vec!["4000"],
            vec!["5000", "6000"],
            vec!["7000", "8000", "9000"],
            vec!["10000"]];
        assert_eq!(res, want);
    }

    #[test]
    fn elf_cals() {
        let elves = vec![
            vec!["1000".to_string(), "2000".to_string(), "3000".to_string()],
            vec!["4000".to_string()],
            vec!["5000".to_string(), "6000".to_string()],
            vec!["7000".to_string(), "8000".to_string(), "9000".to_string()],
            vec!["10000".to_string()]];
        let want: Vec<isize> = vec![6000, 4000, 11000, 24000, 10000];
        let mut result: Vec<isize> = Vec::new();
        for elf in elves {
            result.push(super::elf_cals(elf));
        }
        assert_eq!(want, result);
    }
}

// struct TopThree {
//     Elves: [isize; 3]
// }

// impl TopThree {
//     fn init() -> TopThree {
//         TopThree { Elves: [0, 0, 0] }
//     }

//     fn insert(&self, input: isize) {
//         let mut to_input = input;
//         for elf in self.Elves {
//             if to_input >= elf {
//                 let tmp = elf;
//                 elf = to_input;
//                 to_input = tmp;
//             }
//         }
//     }
// }

// pub fn part2() -> [isize; 3] {
//     println!("  Part Two:");
//     let mut ans: [isize; 3] = [0, 0, 0];
//     if let Ok(lines) = read_lines("./src/day1/input.txt") {
//         loop {
//             let elfR = parse_elf_cals(&lines);
//             if let Ok(elf) = elfR {

//             }
//         }
//     }
//     ans
// }

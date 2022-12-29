// Elves go on foot
// taking inventory of supplies - puzzle input is number of calories each elf is carrying
// one item per line, \n separated. Number is amount of calories in item.
// blank line == new elf
// Goal - how many calories are being carried by the elf with the MOST calories?

// Steps
// read input file line by line

use std::{fs::File, time::Instant, time::Duration};
use super::util::read_lines;
use std::io::{self};
use super::util::get_filename;


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

pub fn part1(input: super::Data) -> (isize, Duration) {
    let now = Instant::now();
    let mut max: isize = 0;
    let file = get_filename("day1", input);
    let lines = read_lines(file);
    for elf in lines_to_elves(lines) {
        let cals = elf_cals(elf);
        if cals > max {
            max = cals;
        }
    }
    (max, now.elapsed())
}

#[derive(Debug, PartialEq)]
struct TopThree {
    elves: [isize; 3]
}

impl TopThree {
    fn init() -> TopThree {
        TopThree { elves: [0, 0, 0] }
    }

    fn insert(&mut self, input: isize) {
        let mut to_input = input;
        let mut elves: [isize; 3] = [0, 0, 0];
        for (idx, elf) in self.elves.iter().enumerate() {
            if to_input >= *elf {
                elves[idx] = to_input;
                to_input = *elf;
            } else {
                elves[idx] = *elf;
            }
        }
        self.elves = elves;
    }

    fn sum(&self) -> isize {
        let mut res = 0;
        for elf in self.elves{
            res += elf;
        }
        res
    }
}

pub fn part2(input: super::Data) -> (isize, Duration) {
    let now = Instant::now();
    let mut tt = TopThree::init();
    let file = get_filename("day1", input);
    let lines = read_lines(file);
    for elf in lines_to_elves(lines) {
        let cals = elf_cals(elf);
        tt.insert(cals)
    }
    (tt.sum(), now.elapsed())
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::Data;
    #[test]
    fn test_part1() {
        let (res, _dur) = part1(Data::Test);
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

    #[test]
    fn topthree_init() {
        let res = TopThree::init();
        let want = TopThree{elves: [0, 0, 0]};
        assert_eq!(want, res);
    }

    #[test]
    fn topthree_insert() {
        let mut tt = TopThree{elves: [20, 12, 8]};
        tt.insert(16);
        assert_eq!(TopThree{elves: [20, 16, 12]}, tt);
    }

    #[test]
    fn topthree_insert_lower() {
        let mut tt = TopThree{elves: [20, 12, 8]};
        tt.insert(6);
        assert_eq!(TopThree{elves: [20, 12, 8]}, tt);
    }

    #[test]
    fn topthree_sum() {
        let mut tt = TopThree::init();
        assert_eq!(tt.sum(), 0);
        tt.insert(1);
        tt.insert(2);
        tt.insert(3);
        assert_eq!(tt.sum(), 6);
        tt.insert(2);
        assert_eq!(tt.sum(), 7);
    }

    #[test]
    fn part_two() {
        let (res, _dur) = part2(Data::Test);
        let want: isize = 45000;
        assert_eq!(want, res);
    }
}

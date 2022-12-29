// use super::util::*;

use super::util::read_lines;
use super::util::get_filename;
use std::time::{Instant, Duration};

// Split the rucksack into its two compartments
fn split_compartments(rucksack: &str) -> Vec<&str> {
    let len = rucksack.len();
    let (comp1, comp2) = rucksack.split_at(len/2);
    return vec![comp1, comp2];
}

// Exactly one item in each rucksack appears in both compartments
fn find_repeat(rucksack: Vec<&str>) -> char {
    let mut comp1:Vec<char> = rucksack[0].chars().collect::<Vec<char>>();
    let mut comp2:Vec<char> = rucksack[1].chars().collect::<Vec<char>>();
    comp1.sort_by(|a, b| a.cmp(b));
    comp2.sort_by(|a, b| a.cmp(b));
    // println!("{}", comp1.clone().iter().collect::<String>());
    // println!("{}", comp2.clone().iter().collect::<String>());

    let mut i = 0;
    let mut j = 0;
    loop {
        // println!("  Comparing {} and {}...", comp1[i], comp2[j]);
        if i > comp1.len() || j > comp2.len() {
            return ' ';
        } else if comp1[i] == comp2[j] {
            return comp1[i];
        } else if comp2[j] > comp1[i] {
            i += 1;
        } else if comp2[j] < comp1[i] {
            j += 1;
        }
    }
}

fn get_priority(letter: char) -> usize {
    let mut priority: usize = 0;
    if letter.is_uppercase() {
        priority += 26;
    }
    let priority_map = "abcdefghijklmnopqrstuvwxyz".to_string();
    for (idx, value) in priority_map.chars().enumerate() {
        if letter.to_ascii_lowercase() == value {
            priority += idx + 1;
        }
    }

    priority
}

fn next_item(elf: Vec<char>, idx: usize) -> usize {
    let mut new_idx = idx;
    while new_idx < elf.len() {
        if elf[new_idx] != elf[idx] {
            return new_idx;
        } 
        new_idx +=1;
    }
    return elf.len()-1;
}

fn get_badge(elf1: Vec<char>, elf2: Vec<char>, elf3: Vec<char>) -> char {
    let (mut i, mut j, mut k) = (0, 0, 0);

    loop {
        // println!("  Comparing {}, {} and {}", elf1[i], elf2[j], elf3[k]);
        if i >= elf1.len() || j >= elf2.len() || k >= elf3.len() {
            break;
        }
        if elf1[i] == elf2[j] && elf2[j] == elf3[k] {
            return elf1[i];
        } else {
            let current_items = vec![elf1[i], elf2[j], elf3[k]];
            let min = current_items.iter().min();
            if let Some(badge) = min{
                if elf1[i] == *badge {
                    i = next_item(elf1.clone(), i);
                } else if elf2[j] == *badge {
                    j = next_item(elf2.clone(), j);
                } else if elf3[k] == *badge {
                    k = next_item(elf3.clone(), k);
                }
            }
        }
    }
    return ' '
}

pub fn part1(input: super::Data) -> (usize, Duration) {
    let now = Instant::now();
    let file = read_lines(get_filename("day3", input));
    let mut res = 0;
    if let Ok(lines) = file {
        for line in lines {
            if let Ok(line) = line {
                let repeat = find_repeat(split_compartments(line.as_str()));
                res += get_priority(repeat);
            }
        }
    }
    (res, now.elapsed())
}

pub fn part2(input: super::Data) -> (usize, Duration) {
    let now = Instant::now();
    // Get file
    let file = read_lines(get_filename("day3", input));
    let mut pri = 0;
    if let Ok(mut lines) = file {
        loop {
            let elf1 = lines.next();
            let elf2 = lines.next();
            let elf3 = lines.next();
            if let None = elf1 {
                break;
            }
            // Convert to char vectors
            let mut elf1 = elf1.unwrap().unwrap().chars().collect::<Vec<char>>();
            let mut elf2 = elf2.unwrap().unwrap().chars().collect::<Vec<char>>();
            let mut elf3 = elf3.unwrap().unwrap().chars().collect::<Vec<char>>();

            // Sort
            elf1.sort_by(|a, b| a.cmp(b));
            elf2.sort_by(|a, b| a.cmp(b));
            elf3.sort_by(|a, b| a.cmp(b));
            // println!(
            //     "{}\n{}\n{}",
            //     elf1.clone().iter().collect::<String>(),
            //     elf2.clone().iter().collect::<String>(),
            //     elf3.clone().iter().collect::<String>()
            // );
            let badge = get_badge(elf1, elf2, elf3);
            pri += get_priority(badge);
        }
    }
    (pri, now.elapsed())
}

#[cfg(test)]
mod tests {
    #[test]
    fn split_compartments() {
        let want = vec!["abcd", "efgh"];
        let res = super::split_compartments("abcdefgh");
        assert_eq!(want, res);
    }
    
    #[test]
    fn find_repeat() {
        let want = 'c';
        let res = super::find_repeat(vec!["abEcd", "BRcFG"]);
        assert_eq!(want, res);
    }
}
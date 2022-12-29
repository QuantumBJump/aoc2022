mod day1;
mod day2;
mod day3;
mod day4;
pub mod util;

#[cfg(test)]
extern crate table_test;
pub enum Data {
    Input,
    Test
}
fn main() {
    println!("Hello, world!");
    println!("Day 1:");
    let (ans, dur)= day1::part1(Data::Input);
    println!("  Part 1: {} in {:.2?}", ans, dur);
    let (ans, dur) = day1::part2(Data::Input);
    println!("  Part 2: {} in {:.2?}", ans, dur);

    println!("Day 2:");
    let (ans, dur) = day2::part1(Data::Input);
    println!("  Part 1: {} in {:.2?}", ans, dur);
    let (ans, dur) = day2::part2(Data::Input);
    println!("  Part 2: {} in {:.2?}", ans, dur);

    println!("Day 3:");
    let (ans, dur) = day3::part1(Data::Input);
    println!("  Part 1: {} in {:.2?}", ans, dur);
    let (ans, dur) = day3::part2(Data::Input);
    println!("  Part 2: {} in {:.2?}", ans, dur);

    println!("Day 4:");
    let (ans, dur) = day4::part1(Data::Input);
    println!("  Part 1: {} in {:.2?}", ans, dur);
    let (ans, dur) = day4::part2(Data::Input);
    println!("  Part 2: {} in {:.2?}", ans, dur);
}

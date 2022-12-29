mod day1;
mod day2;
mod day3;
pub mod util;
pub enum Data {
    Input,
    Test
}
fn main() {
    println!("Hello, world!");
    println!("Day 1:");
    let ans = day1::part1(Data::Input);
    println!("  Part 1: {}", ans);
    let ans = day1::part2(Data::Input);
    println!("  Part 2: {}", ans);

    println!("Day 2:");
    let ans = day2::part1(Data::Input);
    println!("  Part 1: {}", ans);
    let ans = day2::part2(Data::Input);
    println!("  Part 2: {}", ans);

    println!("Day 3:");
    let ans = day3::part1(Data::Input);
    println!("  Part 1: {}", ans);
    let ans = day3::part2(Data::Input);
    println!("  Part 2: {}", ans);
}

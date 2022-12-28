mod day1;
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
}

use super::Data;
use std::io::{BufRead, self};
use std::path::Path;
use std::fs::File;

pub fn get_filename(day: &str, input: Data) -> String {
    let file: &str;
    match input {
        Data::Input => {file = "input.txt"}
        Data::Test => file = "test.txt"
    }
    return format!("./src/{}/{}", day, file);
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day1() {
        let res = get_filename("day1", Data::Test);
        let want =  "./src/day1/test.txt";
        assert_eq!(want, res);
        let res = get_filename("day1", Data::Input);
        let want = "./src/day1/input.txt";
        assert_eq!(want, res);
    }

    #[test]
    fn day2() {
        let res = get_filename("day2", Data::Test);
        let want = "./src/day2/test.txt";
        assert_eq!(want, res);
        let res = get_filename("day2", Data::Input);
        let want = "./src/day2/input.txt";
        assert_eq!(want, res);
    }
}
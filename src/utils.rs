use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;

fn input(file: &str) -> String {
    format!("input/{file}.txt")
}

pub fn read_lines(file: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let input = input(file);
    let path = Path::new(&input);
    let file = File::open(path)?;

    Ok(io::BufReader::new(file).lines())
}

pub fn read_to_string(file: &str) -> io::Result<String> {
    let input = input(file);
    let path = Path::new(&input);
    fs::read_to_string(path)
}

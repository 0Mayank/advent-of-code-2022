use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines(file: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let input = format!("input/{file}.txt");
    let path = Path::new(&input);
    let file = File::open(path)?;

    Ok(io::BufReader::new(file).lines())
}

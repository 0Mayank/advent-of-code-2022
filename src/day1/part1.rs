use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() -> i32 {
    let mut elves = vec![0];
    let mut i = 0;
    if let Ok(lines) = read_lines("src/day1/puzzle_input.txt") {
        for line in lines {
            if let Ok(s) = line {
                if s.is_empty() {
                    i += 1;
                    elves.push(0);
                } else {
                    elves[i] += s.parse::<i32>().unwrap();
                }
            }
        }
    }

    return *elves.iter().max().unwrap();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<std::io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

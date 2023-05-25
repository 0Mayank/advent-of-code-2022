use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn priority_of_common_item(first_compartment: &str, second_compartment: &str) -> u32 {
    let mut f = HashSet::new();
    for x in first_compartment.chars() {
        f.insert(x);
    }

    let mut s = HashSet::new();
    for x in second_compartment.chars() {
        s.insert(x);
    }

    let common = *f.intersection(&s).next().unwrap();

    if common.is_uppercase() == true {
        return common as u32 - 'A' as u32 + 27;
    } else {
        return common as u32 - 'a' as u32 + 1;
    };
}

pub fn run() -> i32 {
    let lines = read_lines("src/day3/puzzle_input.txt").unwrap();

    lines.fold(0, |acc, s| {
        let s = s.unwrap();
        let mid = s.len() / 2;
        return acc + priority_of_common_item(&s[0..mid], &s[mid..]);
    }) as i32
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;

    Ok(io::BufReader::new(file).lines())
}

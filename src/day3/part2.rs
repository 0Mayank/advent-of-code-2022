use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn priority_of_common_item(group: &[String]) -> u32 {
    let common = *group
        .iter()
        .map(|s| {
            let mut h = HashSet::new();
            for x in s.chars() {
                h.insert(x);
            }
            return h;
        })
        .reduce(|acc, c| acc.intersection(&c).map(|x| *x).collect())
        .unwrap()
        .iter()
        .next()
        .unwrap();

    if common.is_uppercase() == true {
        return common as u32 - 'A' as u32 + 27;
    } else {
        return common as u32 - 'a' as u32 + 1;
    };
}

pub fn run() -> i32 {
    let lines = read_lines("src/day3/puzzle_input.txt").unwrap();
    let mut count = 0;
    let mut group: Vec<String> = vec![String::new(); 3];

    lines.fold(0, |acc, s| {
        group[count] = s.unwrap();
        count += 1;
        if count == 3 {
            count = 0;
            return acc + priority_of_common_item(&group);
        }
        return acc;
    }) as i32
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;

    Ok(io::BufReader::new(file).lines())
}

use crate::utils::read_lines;
use std::collections::HashSet;

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

pub fn run() -> String {
    let lines = read_lines("src/day3/puzzle_input.txt").unwrap();
    let mut count = 0;
    let mut group: Vec<String> = vec![String::new(); 3];

    lines
        .fold(0, |acc, s| {
            group[count] = s.unwrap();
            count += 1;
            if count == 3 {
                count = 0;
                return acc + priority_of_common_item(&group);
            }
            return acc;
        })
        .to_string()
}

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
            h
        })
        .reduce(|acc, c| acc.intersection(&c).copied().collect())
        .unwrap()
        .iter()
        .next()
        .unwrap();

    if common.is_uppercase() {
        common as u32 - 'A' as u32 + 27
    } else {
        common as u32 - 'a' as u32 + 1
    }
}

pub fn run() -> String {
    let lines = read_lines("3").unwrap();
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
            acc
        })
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn verify() {
        assert_eq!(2798.to_string(), run());
    }
}

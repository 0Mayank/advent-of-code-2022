use crate::utils::read_lines;
use std::collections::HashSet;

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

    if common.is_uppercase() {
        common as u32 - 'A' as u32 + 27
    } else {
        common as u32 - 'a' as u32 + 1
    }
}

pub fn run() -> String {
    let lines = read_lines("3").unwrap();

    lines
        .fold(0, |acc, s| {
            let s = s.unwrap();
            let mid = s.len() / 2;
            acc + priority_of_common_item(&s[0..mid], &s[mid..])
        })
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn verify() {
        assert_eq!(7824.to_string(), run());
    }
}

use crate::utils::read_lines;

pub fn run() -> String {
    let mut elves = vec![0];
    let mut i = 0;
    if let Ok(lines) = read_lines("1") {
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
    elves.sort();
    elves.reverse();
    let nth_highest = 3;

    return elves[0..nth_highest].iter().sum::<i32>().to_string();
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn verify() {
        assert_eq!(197400.to_string(), run());
    }
}

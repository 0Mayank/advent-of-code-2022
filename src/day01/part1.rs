use crate::utils::read_lines;

pub fn run() -> String {
    let mut elves = vec![0];
    let mut i = 0;
    if let Ok(lines) = read_lines("1") {
        for line in lines.flatten() {
            if line.is_empty() {
                i += 1;
                elves.push(0);
            } else {
                elves[i] += line.parse::<i32>().unwrap();
            }
        }
    }

    return elves.iter().max().unwrap().to_string();
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn verify() {
        assert_eq!(69206.to_string(), run());
    }
}

use crate::utils::read_lines;

pub fn run() -> String {
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

    return elves.iter().max().unwrap().to_string();
}

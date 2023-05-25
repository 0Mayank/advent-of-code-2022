use crate::utils::read_lines;

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
    elves.sort();
    elves.reverse();
    let nth_highest = 3;

    return elves[0..nth_highest].iter().sum();
}

use crate::utils::read_lines;

pub fn run() -> String {
    let lines = read_lines("src/day4/puzzle_input.txt").unwrap();

    lines
        .fold(0, |acc, pair| {
            let pair = pair.unwrap();
            let pair: Vec<_> = pair
                .split(",")
                .map(|s| {
                    let s: Vec<_> = s.split("-").map(|x| x.parse::<i32>().unwrap()).collect();
                    return s;
                })
                .collect();

            let contains = (pair[0][0] >= pair[1][0] && pair[0][1] <= pair[1][1])
                || (pair[0][0] <= pair[1][0] && pair[0][1] >= pair[1][1]);

            if contains {
                return acc + 1;
            }

            return acc;
        })
        .to_string()
}

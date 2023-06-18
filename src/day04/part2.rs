use crate::utils::read_lines;

pub fn run() -> String {
    let lines = read_lines("4").unwrap();

    lines
        .fold(0, |acc, pair| {
            let pair = pair.unwrap();
            let pair: Vec<_> = pair
                .split(',')
                .map(|s| {
                    let s: Vec<_> = s.split('-').map(|x| x.parse::<i32>().unwrap()).collect();
                    s
                })
                .collect();
            let contains = (pair[0][0] >= pair[1][0] && pair[0][1] <= pair[1][1])
                || (pair[0][0] <= pair[1][0] && pair[0][1] >= pair[1][1]);

            let overlap = (pair[0][0] <= pair[1][1] && pair[0][0] >= pair[1][0])
                || (pair[0][1] >= pair[1][0] && pair[0][1] <= pair[1][1]);

            if contains || overlap {
                return acc + 1;
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
        assert_eq!(852.to_string(), run());
    }
}

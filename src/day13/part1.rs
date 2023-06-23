use std::{cmp::Ordering, fmt};

use crate::utils::read_lines;

#[derive(PartialEq, Eq)]
enum Node {
    Num(u32),
    List(Vec<Node>),
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Num(a), Self::Num(b)) => a.cmp(b),
            (Self::List(a), Self::List(b)) => {
                if a.len() != b.len() {
                    return a.cmp(b);
                }
                for (a, b) in a.iter().zip(b) {
                    match a.cmp(b) {
                        Ordering::Equal => continue,
                        other => return other,
                    }
                }
                return Ordering::Equal;
            }
            (Self::Num(a), Self::List(_)) => Self::List(vec![Self::Num(*a)]).cmp(other),
            (Self::List(_), Self::Num(b)) => self.cmp(&Self::List(vec![Self::Num(*b)])),
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Num(n) => write!(f, "{n}"),
            Self::List(n) => f.debug_list().entries(n).finish(),
        }
    }
}

impl TryFrom<&str> for Node {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut stack = vec![];
        let mut num = String::new();

        let push_num_and_clear =
            |num: &mut String, stack: &mut Vec<Node>| -> Result<(), Self::Error> {
                if let Some(Node::List(list)) = stack.last_mut() {
                    if !num.is_empty() {
                        list.push(Node::Num(num.parse().map_err(|e| format!("{e}"))?));
                        num.clear();
                    }
                }

                Ok(())
            };

        for x in value.chars() {
            match x {
                '0'..='9' => num.push(x),
                ',' => {
                    push_num_and_clear(&mut num, &mut stack)?;
                }
                '[' => {
                    let node = Node::List(vec![]);
                    stack.push(node);
                }
                ']' => {
                    push_num_and_clear(&mut num, &mut stack)?;
                    let popped = stack.pop().unwrap();
                    if let Some(Node::List(list)) = stack.last_mut() {
                        list.push(popped);
                    } else {
                        return Ok(popped);
                    }
                }
                other => return Err(format!("Invalid character {other}")),
            }
        }

        Err("Invalid String".to_string())
    }
}

pub fn run() -> String {
    let mut pairs: Vec<Node> = vec![];

    for line in read_lines("13").unwrap().flatten() {
        if line.is_empty() {
            continue;
        }
        pairs.push(line.as_str().try_into().unwrap());
    }

    let pairs = std::iter::zip(pairs.iter().step_by(2), pairs.iter().skip(1).step_by(2));

    pairs
        .enumerate()
        .fold(0, |acc, (i, pair)| {
            if pair.0 < pair.1 {
                return acc + i + 1;
            }
            return acc;
        })
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn verify() {
        assert_eq!(5196.to_string(), run());
    }
}

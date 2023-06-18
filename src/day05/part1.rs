use std::collections::{HashMap, VecDeque};

use crate::utils::read_lines;

#[derive(Debug)]
struct Supplies {
    stacks: HashMap<usize, VecDeque<String>>,
}

impl Supplies {
    fn new() -> Self {
        Self {
            stacks: HashMap::new(),
        }
    }

    fn add(&mut self, p: &(usize, &str)) {
        let (i, c) = *p;

        match self.stacks.get_mut(&i) {
            Some(stack) => stack.push_front(c.to_string()),
            None => {
                let mut v = VecDeque::new();
                v.push_front(c.to_string());
                self.stacks.insert(i, v);
            }
        };
    }

    fn mv(&mut self, quantity: usize, from: usize, to: usize) {
        for _ in 0..quantity {
            if let Some(last) = self.stacks.get_mut(&from).unwrap().pop_back() {
                self.stacks.get_mut(&to).unwrap().push_back(last);
            }
        }
    }

    fn top_of_stacks(&self) -> String {
        let mut msg = String::new();
        for stack in 1..=self.stacks.len() {
            msg += self.stacks.get(&stack).unwrap().back().unwrap();
        }

        msg
    }
}

pub fn run() -> String {
    let mut lines = read_lines("5").unwrap();
    let mut supplies = Supplies::new();

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let line = line.split(&[' ', '[', ']'][..]);
        if !line
            .clone()
            .any(|x| x.chars().all(|c| !c.is_ascii_digit()) && !x.is_empty())
        {
            lines.next();
            break;
        }

        let mut consecutive_spaces = 0;
        line.filter(|c| {
            if !c.is_empty() {
                consecutive_spaces = 0;
                return true;
            }
            consecutive_spaces += 1;

            if consecutive_spaces == 4 {
                consecutive_spaces = 0;
                return true;
            }
            false
        })
        .enumerate()
        .filter(|(_, c)| !c.is_empty())
        .for_each(|(i, c)| supplies.add(&(i + 1, c)));
    }

    for line in lines {
        let line = line.unwrap();
        let line: Vec<_> = line
            .split_whitespace()
            .filter_map(|c| c.parse::<usize>().ok())
            .collect();
        supplies.mv(line[0], line[1], line[2]);
    }
    supplies.top_of_stacks()
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn verify() {
        assert_eq!("LBLVVTVLP".to_string(), run());
    }
}

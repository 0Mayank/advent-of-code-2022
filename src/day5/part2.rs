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
        let mut to_move = VecDeque::new();
        for _ in 0..quantity {
            if let Some(last) = self.stacks.get_mut(&from).unwrap().pop_back() {
                to_move.push_front(last);
            }
        }
        for _ in 0..quantity {
            if let Some(front) = to_move.pop_front() {
                self.stacks.get_mut(&to).unwrap().push_back(front);
            }
        }
    }

    fn top_of_stacks(&self) -> String {
        let mut msg = String::new();
        for stack in 1..=self.stacks.len() {
            msg += self.stacks.get(&stack).unwrap().back().unwrap();
        }

        return msg;
    }
}

pub fn run() -> String {
    let mut lines = read_lines("src/day5/puzzle_input.txt").unwrap();
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
            return false;
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
    return supplies.top_of_stacks();
}

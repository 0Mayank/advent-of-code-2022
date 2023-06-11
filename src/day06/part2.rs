use std::{
    collections::VecDeque,
    ops::{Deref, DerefMut},
};

use crate::utils::read_lines;

struct SlidingWindow {
    value: VecDeque<char>,
    index: usize,
}

impl Deref for SlidingWindow {
    type Target = VecDeque<char>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for SlidingWindow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl SlidingWindow {
    fn new() -> Self {
        Self {
            value: VecDeque::new(),
            index: 0,
        }
    }

    fn push_and_test_for_message_marker(&mut self, c: &char) -> Option<usize> {
        self.index += 1;
        match self.iter().position(|&x| x == *c) {
            Some(i) => {
                for _ in 0..=i {
                    self.pop_front();
                }
                self.push_back(*c);
                return None;
            }
            None => {
                self.push_back(*c);
                if self.len() == 14 {
                    return Some(self.index);
                }
                return None;
            }
        }
    }
}

pub fn run() -> String {
    let datastream = read_lines("6").unwrap().next().unwrap().unwrap();
    let mut window = SlidingWindow::new();

    for c in datastream.chars() {
        if let Some(i) = window.push_and_test_for_message_marker(&c) {
            return i.to_string();
        }
    }

    return "Start-of-message marker not found".to_string();
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn verify() {
        assert_eq!(3380.to_string(), run());
    }
}

use std::{
    collections::HashSet,
    ops::{AddAssign, Sub},
};

use crate::utils::read_lines;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coords {
    x: i32,
    y: i32,
}

impl Coords {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn abs(&self) -> f32 {
        ((self.x.pow(2) + self.y.pow(2)) as f32).sqrt()
    }
}

impl AddAssign for Coords {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Coords {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Debug)]
struct Rope {
    body: Vec<Coords>,
    visited: HashSet<Coords>,
}

impl Rope {
    fn new() -> Self {
        Self {
            body: vec![Coords::new(0, 0); 10],
            visited: HashSet::from([Coords::new(0, 0)]),
        }
    }

    fn head_mut(&mut self) -> &mut Coords {
        &mut self.body[0]
    }

    fn tail(&self) -> &Coords {
        &self.body.last().unwrap()
    }

    fn right(&mut self, by: i32) {
        for _ in 0..by {
            self.head_mut().x += 1;
            self.move_body();
        }
    }

    fn left(&mut self, by: i32) {
        for _ in 0..by {
            self.head_mut().x -= 1;
            self.move_body();
        }
    }

    fn up(&mut self, by: i32) {
        for _ in 0..by {
            self.head_mut().y += 1;
            self.move_body();
        }
    }

    fn down(&mut self, by: i32) {
        for _ in 0..by {
            self.head_mut().y -= 1;
            self.move_body();
        }
    }

    fn move_body(&mut self) {
        let move_logic = |head: Coords, tail: &mut Coords| {
            let mut diff = head - *tail;
            let distance = diff.abs();

            if distance <= f32::sqrt(2f32) {
                diff.x = 0;
                diff.y = 0;
            } else {
                diff.x = diff.x.signum();
                diff.y = diff.y.signum();
            }

            *tail += diff;
        };

        for i in 1..self.body.len() {
            let head = self.body[i - 1];
            let tail = &mut self.body[i];
            move_logic(head, tail);
        }

        self.visited.insert(*self.tail());
    }
}

pub fn run() -> String {
    let lines = read_lines("9").unwrap();
    let mut rope = Rope::new();

    for line in lines {
        let line = line.unwrap();
        let mut line = line.split_whitespace();
        match line.next().unwrap() {
            "R" => rope.right(line.next().unwrap().parse().unwrap()),
            "L" => rope.left(line.next().unwrap().parse().unwrap()),
            "U" => rope.up(line.next().unwrap().parse().unwrap()),
            "D" => rope.down(line.next().unwrap().parse().unwrap()),
            other => panic!("Invalid direction: {other}"),
        }
    }
    rope.visited.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn verify() {
        assert_eq!(2504.to_string(), run());
    }
}

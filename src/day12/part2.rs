use std::{
    collections::{BinaryHeap, HashSet},
    ops::Index,
};

use crate::utils::read_to_string;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn neighbours(&self, rows: usize, cols: usize) -> Vec<Self> {
        let mut result = vec![];

        if self.y > 0 {
            result.push(Coord {
                x: self.x,
                y: self.y - 1,
            });
        }

        if self.x > 0 {
            result.push(Coord {
                x: self.x - 1,
                y: self.y,
            });
        }

        if self.y < rows - 1 {
            result.push(Coord {
                x: self.x,
                y: self.y + 1,
            });
        }

        if self.x < cols - 1 {
            result.push(Coord {
                x: self.x + 1,
                y: self.y,
            });
        }

        result
    }
}

impl From<(usize, usize)> for Coord {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Node {
    distance: u32,
    coords: Coord,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.distance.cmp(&self.distance))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance)
    }
}

struct Grid {
    rows: usize,
    cols: usize,
    start: Coord,
    target: u8,
    data: Vec<u8>,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let mut grid = Self {
            rows: input.lines().count(),
            cols: input.lines().next().unwrap().len(),
            start: Coord { x: 0, y: 0 },
            target: 0,
            data: Default::default(),
        };

        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                let elevation;
                match c {
                    'S' => elevation = 0,
                    'E' => {
                        grid.start = (col, row).into();
                        elevation = 25
                    }
                    'a'..='z' => elevation = c as u8 - b'a',
                    _ => panic!("Invalid character"),
                }
                grid.data.push(elevation);
            }
        }

        grid
    }

    fn valid_candidates(&self, current: &Coord) -> Vec<Coord> {
        current
            .neighbours(self.rows, self.cols)
            .iter()
            .filter_map(|neighbour| {
                if self[current] > self[neighbour] + 1 {
                    return None;
                }
                return Some(*neighbour);
            })
            .collect()
    }

    fn shortest_path_from_e_to_elevation_a(&self) -> u32 {
        let mut priority_queue = BinaryHeap::new();
        let mut visited = HashSet::new();

        priority_queue.push(Node {
            distance: 0,
            coords: self.start,
        });

        visited.insert(self.start);

        while let Some(Node { distance, coords }) = priority_queue.pop() {
            if self[&coords] == self.target {
                return distance;
            }

            for candidate in self.valid_candidates(&coords) {
                if visited.insert(candidate) {
                    priority_queue.push(Node {
                        distance: distance + 1,
                        coords: candidate,
                    })
                }
            }
        }

        panic!("No path found");
    }
}

impl Index<&Coord> for Grid {
    type Output = u8;

    fn index(&self, coords: &Coord) -> &Self::Output {
        &self.data[coords.y * self.cols + coords.x]
    }
}

pub fn run() -> String {
    let grid = Grid::parse(&read_to_string("12").unwrap());
    grid.shortest_path_from_e_to_elevation_a().to_string()
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn verify() {
        assert_eq!(416.to_string(), run());
    }
}

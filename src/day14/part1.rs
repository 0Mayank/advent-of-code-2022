use std::fmt;

use crate::utils::read_to_string;

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Falling,
    Rest,
}

#[derive(Clone, Copy)]
enum Tile {
    Air,
    Rock,
    Sand(State),
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Air => write!(f, "."),
            Self::Rock => write!(f, "#"),
            Self::Sand(_) => write!(f, "o"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord {
    x: i32,
    y: i32,
}

impl TryFrom<&str> for Coord {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut value = value.split(',');
        Ok(Self {
            x: value
                .next()
                .ok_or("Invalid Input".to_string())?
                .parse()
                .map_err(|e| format!("{e}"))?,
            y: value
                .next()
                .ok_or("Invalid Input".to_string())?
                .parse()
                .map_err(|e| format!("{e}"))?,
        })
    }
}

#[derive(Debug)]
struct Grid {
    data: Vec<Vec<Tile>>,
    offset: Coord,
    limit: Coord,
    source: Coord,
}

impl Grid {
    fn parse(input: &str) -> Result<Self, String> {
        let rocks: Vec<_> = input
            .lines()
            .map(|x| Self::polyline(x.split(" -> ").map(|coord| coord.try_into().unwrap())))
            .flatten()
            .collect();

        let source = Coord { x: 500, y: 0 };

        let offset = Coord {
            x: (rocks.iter().min_by(|a, b| a.x.cmp(&b.x)).unwrap().x).min(source.x),
            y: (rocks.iter().min_by(|a, b| a.y.cmp(&b.y)).unwrap().y).min(source.y),
        };

        let limit = Coord {
            x: (rocks.iter().max_by(|a, b| a.x.cmp(&b.x)).unwrap().x + 1).max(source.x),
            y: (rocks.iter().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y + 1).max(source.y),
        };

        let mut res = Self {
            data: vec![
                vec![Tile::Air; (limit.x - offset.x) as usize];
                (limit.y - offset.y) as usize
            ],
            offset,
            limit,
            source,
        };

        for rock in rocks {
            *res.get_mut(&rock).unwrap() = Tile::Rock;
        }
        Ok(res)
    }

    fn polyline(mut line: impl Iterator<Item = Coord>) -> Vec<Coord> {
        let mut start = line.next().expect("Line is empty");
        let mut res = vec![start];

        for point in line {
            if point.x > start.x {
                for i in (start.x + 1)..=point.x {
                    res.push(Coord { x: i, y: point.y });
                }
            } else if point.x < start.x {
                for i in point.x..start.x {
                    res.push(Coord { x: i, y: point.y });
                }
            } else if point.y > start.y {
                for i in (start.y + 1)..=point.y {
                    res.push(Coord { x: point.x, y: i });
                }
            } else if point.y < start.y {
                for i in point.y..start.y {
                    res.push(Coord { x: point.x, y: i });
                }
            }

            start = point
        }

        res
    }

    fn fall(&mut self, sand: &mut Coord) -> State {
        loop {
            if self.is_abyss(sand) {
                if let Some(sand) = self.get_mut(sand) {
                    *sand = Tile::Air
                }
                return State::Falling;
            }

            let down = Coord {
                x: sand.x,
                y: sand.y + 1,
            };
            let down_left = Coord {
                x: sand.x - 1,
                y: sand.y + 1,
            };
            let down_right = Coord {
                x: sand.x + 1,
                y: sand.y + 1,
            };

            let flow_dir = vec![down, down_left, down_right]
                .iter()
                .find(|&&x| {
                    if let Some(Tile::Air) = self.get(&x) {
                        return true;
                    }
                    false
                })
                .copied();

            if let Some(flow_dir) = flow_dir {
                *self.get_mut(&flow_dir).unwrap() = Tile::Sand(State::Falling);
                *self.get_mut(sand).unwrap() = Tile::Air;
                *sand = flow_dir;
            } else {
                *self.get_mut(sand).unwrap() = Tile::Sand(State::Rest);
                break;
            }
        }
        return State::Rest;
    }

    fn simulate(&mut self) {
        let mut sand_pos = self.source;
        while let State::Rest = self.fall(&mut sand_pos) {
            sand_pos = self.source;
        }
    }

    fn is_abyss(&self, coords: &Coord) -> bool {
        coords.y >= self.limit.y - 1 || coords.x <= self.offset.x || coords.x >= self.limit.x - 1
    }

    fn get(&self, coords: &Coord) -> Option<&Tile> {
        self.data
            .get((coords.y - self.offset.y) as usize)?
            .get((coords.x - self.offset.x) as usize)
    }

    fn get_mut(&mut self, coords: &Coord) -> Option<&mut Tile> {
        self.data
            .get_mut((coords.y - self.offset.y) as usize)?
            .get_mut((coords.x - self.offset.x) as usize)
    }
}

pub fn run() -> String {
    let mut grid = Grid::parse(&read_to_string("14").unwrap()).unwrap();
    grid.simulate();

    grid.data
        .iter()
        .flatten()
        .filter(|x| {
            if let Tile::Sand(State::Rest) = x {
                true
            } else {
                false
            }
        })
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn verify() {
        assert_eq!(1072.to_string(), run());
    }
}

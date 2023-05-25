use crate::utils::read_lines;

struct Turn {
    opponent: Move,
    you: Move,
}

#[derive(PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<String> for Turn {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut ip = value.split(' ');

        let opp: Move = if let Some(s) = ip.next() {
            s.try_into()?
        } else {
            return Err(format!("No move present for Opponent"));
        };

        let you: Move = if let Some(s) = ip.next() {
            s.try_into()?
        } else {
            return Err(format!("No move present for You"));
        };

        Ok(Self { opponent: opp, you })
    }
}

impl Turn {
    fn points(&self) -> i32 {
        let mut p = 0;

        if self.you.win_condition() == self.opponent {
            p += 6;
        } else if self.you == self.opponent {
            p += 3;
        }

        p += self.you.points();
        return p;
    }
}

impl TryFrom<&str> for Move {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            other => Err(format!("{} is not a valid move.", other)),
        }
    }
}

impl Move {
    fn points(&self) -> i32 {
        match *self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn win_condition(&self) -> Self {
        match *self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }
}

pub fn run() -> i32 {
    let mut total_points = 0;

    if let Ok(lines) = read_lines("src/day2/puzzle_input.txt") {
        for line in lines {
            if let Ok(line) = line {
                let turn: Turn = line.clone().try_into().unwrap();
                total_points += turn.points();
            }
        }
    }

    return total_points;
}

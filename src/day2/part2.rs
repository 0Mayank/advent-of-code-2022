use crate::utils::read_lines;

struct Turn {
    opponent: Move,
    turn_result: TurnResult,
}

#[derive(PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

enum TurnResult {
    Lose,
    Draw,
    Win,
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

        let you: TurnResult = if let Some(s) = ip.next() {
            s.try_into()?
        } else {
            return Err(format!("No turn result present for You"));
        };

        Ok(Self {
            opponent: opp,
            turn_result: you,
        })
    }
}

impl Turn {
    fn points(&self) -> i32 {
        let mut p = self.turn_result.points();

        match self.turn_result {
            TurnResult::Lose => p += self.opponent.win_condition().points(),
            TurnResult::Draw => p += self.opponent.points(),
            TurnResult::Win => p += self.opponent.lose_condition().points(),
        }
        return p;
    }
}

impl TryFrom<&str> for Move {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
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

    fn lose_condition(&self) -> Self {
        match *self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }
}

impl TryFrom<&str> for TurnResult {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            other => Err(format!("{} is not a valid result of a turn.", other)),
        }
    }
}

impl TurnResult {
    fn points(&self) -> i32 {
        match *self {
            Self::Win => 6,
            Self::Lose => 0,
            Self::Draw => 3,
        }
    }
}

pub fn run() -> String {
    let mut total_points = 0;

    if let Ok(lines) = read_lines("src/day2/puzzle_input.txt") {
        for line in lines {
            if let Ok(line) = line {
                let turn: Turn = line.try_into().unwrap();
                total_points += turn.points();
            }
        }
    }

    return total_points.to_string();
}

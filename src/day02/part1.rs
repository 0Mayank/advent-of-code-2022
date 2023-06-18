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
            return Err("No move present for Opponent".to_string());
        };

        let you: Move = if let Some(s) = ip.next() {
            s.try_into()?
        } else {
            return Err("No move present for You".to_string());
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
        p
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

pub fn run() -> String {
    let mut total_points = 0;

    if let Ok(lines) = read_lines("2") {
        for line in lines.flatten() {
            let turn: Turn = line.clone().try_into().unwrap();
            total_points += turn.points();
        }
    }

    total_points.to_string()
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn verify() {
        assert_eq!(13809.to_string(), run());
    }
}

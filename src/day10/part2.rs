use std::ops::{Deref, DerefMut};

use crate::utils::read_lines;

#[derive(Debug)]
enum Instruction {
    Addx,
    Noop,
}

impl Instruction {
    fn cycles(&self) -> i32 {
        match self {
            Self::Addx => 2,
            Self::Noop => 1,
        }
    }
}

impl TryFrom<&str> for Instruction {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "addx" => Ok(Self::Addx),
            "noop" => Ok(Self::Noop),
            other => Err(format!("Invalid instruction: {other}")),
        }
    }
}

struct Register {
    x: i32,
}

struct Crt {
    value: Vec<Vec<char>>,
    current_row: usize,
    current_pixel: usize,
}

impl Crt {
    fn new() -> Self {
        Crt {
            value: vec![vec!['.'; 40]; 6],
            current_pixel: 0,
            current_row: 0,
        }
    }
}

impl Deref for Crt {
    type Target = Vec<Vec<char>>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for Crt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

struct Cpu {
    instruction: Instruction,
    instruction_cycle: i32,
    ongoing_cycle: i32,
    registers: Register,
    crt: Crt,
}

impl Cpu {
    fn new() -> Self {
        Self {
            instruction: Instruction::Noop,
            instruction_cycle: 1,
            ongoing_cycle: 1,
            registers: Register { x: 1 },
            crt: Crt::new(),
        }
    }

    fn execution_loop(&mut self, input: &str) -> Result<(), String> {
        self.ongoing_cycle += 1;
        self.draw();
        self.feed(input)?;
        Ok(())
    }

    fn feed(&mut self, input: &str) -> Result<(), String> {
        if self.is_instruction_finished() {
            self.instruction = input.try_into()?;
            self.instruction_cycle = 0;
        } else {
            let cur_instruction = &self.instruction;
            match cur_instruction {
                Instruction::Addx => {
                    let num = if let Ok(num) = input.parse::<i32>() {
                        num
                    } else {
                        return Err(String::from("ADDX needs a number."));
                    };
                    self.registers.x += num;
                }
                Instruction::Noop => {}
            }
        }

        self.instruction_cycle += 1;
        Ok(())
    }

    fn draw(&mut self) {
        let row = self.crt.current_row;
        let pixel = self.crt.current_pixel;
        if (self.registers.x - (pixel as i32)).abs() <= 1 {
            self.crt[row][pixel] = '#';
        } else {
            self.crt[row][pixel] = '.';
        }

        self.crt.current_pixel = (pixel + 1) % self.crt[0].len();
        self.crt.current_row += (pixel + 1) / 40;
    }

    fn is_instruction_finished(&self) -> bool {
        self.instruction_cycle == self.instruction.cycles()
    }
}

pub fn run() -> String {
    let lines = read_lines("10").unwrap();
    let mut cpu = Cpu::new();

    lines.for_each(|x| {
        let line = x.unwrap();
        let line = line.split_whitespace();
        for word in line {
            cpu.execution_loop(word).unwrap();
        }
    });

    let mut output = String::new();

    for row in cpu.crt.iter() {
        for pixel in row {
            output.push(*pixel);
        }
        output.push('\n');
    }

    output
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn verify() {
        let exp = "\
####.###....##.###..###..#..#..##..#..#.
#....#..#....#.#..#.#..#.#.#..#..#.#..#.
###..#..#....#.###..#..#.##...#..#.####.
#....###.....#.#..#.###..#.#..####.#..#.
#....#....#..#.#..#.#.#..#.#..#..#.#..#.
####.#.....##..###..#..#.#..#.#..#.#..#.
";
        assert_eq!(exp.to_string(), run());
    }
}

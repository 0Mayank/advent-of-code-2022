use crate::utils::read_lines;

#[derive(Debug)]
enum Instruction {
    ADDX,
    NOOP,
}

impl Instruction {
    fn cycles(&self) -> i32 {
        match self {
            Self::ADDX => 2,
            Self::NOOP => 1,
        }
    }
}

impl TryFrom<&str> for Instruction {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "addx" => Ok(Self::ADDX),
            "noop" => Ok(Self::NOOP),
            other => Err(format!("Invalid instruction: {other}")),
        }
    }
}

struct Register {
    x: i32,
}

struct CPU {
    instruction: Instruction,
    instruction_cycle: i32,
    ongoing_cycle: i32,
    registers: Register,
}

impl CPU {
    fn new() -> Self {
        Self {
            instruction: Instruction::NOOP,
            instruction_cycle: 1,
            ongoing_cycle: 1,
            registers: Register { x: 1 },
        }
    }

    fn feed(&mut self, input: &str) -> Result<(), String> {
        self.ongoing_cycle += 1;

        if self.is_instruction_finished() {
            self.instruction = input.try_into()?;
            self.instruction_cycle = 0;
        } else {
            let cur_instruction = &self.instruction;
            match cur_instruction {
                Instruction::ADDX => {
                    let num = if let Ok(num) = input.parse::<i32>() {
                        num
                    } else {
                        return Err(String::from("ADDX needs a number."));
                    };
                    self.registers.x += num;
                }
                Instruction::NOOP => {}
            }
        }

        self.instruction_cycle += 1;
        Ok(())
    }

    fn is_instruction_finished(&self) -> bool {
        self.instruction_cycle == self.instruction.cycles()
    }
    fn signal_strength(&self) -> i32 {
        self.ongoing_cycle * self.registers.x
    }
}

pub fn run() -> String {
    let lines = read_lines("src/day10/puzzle_input.txt").unwrap();
    let mut cpu = CPU::new();

    lines
        .fold(0, |acc, x| {
            let line = x.unwrap();
            let line = line.split_whitespace();
            let mut res = 0;
            for word in line {
                cpu.feed(word).unwrap();

                if (cpu.ongoing_cycle - 20) % 40 == 0 {
                    res += cpu.signal_strength();
                }
            }

            return acc + res;
        })
        .to_string()
}

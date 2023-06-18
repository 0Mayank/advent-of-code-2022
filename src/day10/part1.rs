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

struct Cpu {
    instruction: Instruction,
    instruction_cycle: i32,
    ongoing_cycle: i32,
    registers: Register,
}

impl Cpu {
    fn new() -> Self {
        Self {
            instruction: Instruction::Noop,
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

    fn is_instruction_finished(&self) -> bool {
        self.instruction_cycle == self.instruction.cycles()
    }
    fn signal_strength(&self) -> i32 {
        self.ongoing_cycle * self.registers.x
    }
}

pub fn run() -> String {
    let lines = read_lines("10").unwrap();
    let mut cpu = Cpu::new();

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

            acc + res
        })
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn verify() {
        assert_eq!(11820.to_string(), run());
    }
}

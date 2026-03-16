use std::{collections::HashSet, str::FromStr};

use anyhow::{Result, anyhow, bail};

pub fn solve_part_1(input: &str) -> Result<isize> {
    let mut program: Program = input.parse()?;
    let _ = program.run();
    Ok(program.accumulator)
}

pub fn solve_part_2(input: &str) -> Result<isize> {
    let program: Program = input.parse()?;
    program.accumulator_after_fix()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Program {
    instructions: Vec<Instruction>,
    position: usize,
    accumulator: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RunResult {
    Repeated,
    Terminated,
}

impl Program {
    fn accumulator_after_fix(&self) -> Result<isize> {
        for i in 0..self.instructions.len() {
            let Some(swapped) = self.instructions[i].swapped() else {
                continue;
            };

            let mut candidate = self.clone();

            candidate.instructions[i] = swapped;

            if matches!(candidate.run(), RunResult::Terminated) {
                return Ok(candidate.accumulator);
            }
        }

        bail!("no terminating mutation found")
    }

    /// Runs the program until it loops or terminates.
    fn run(&mut self) -> RunResult {
        let mut seen_positions = HashSet::new();

        while self.position < self.instructions.len() && seen_positions.insert(self.position) {
            self.step();
        }

        if self.position == self.instructions.len() {
            RunResult::Terminated
        } else {
            RunResult::Repeated
        }
    }

    fn step(&mut self) {
        let Self {
            instructions,
            position,
            accumulator,
        } = self;

        if *position == instructions.len() {
            return;
        }

        let Instruction {
            operation,
            argument,
        } = instructions[*position];

        match operation {
            Operation::Acc => {
                *accumulator += argument;
                *position += 1;
            }
            Operation::Jmp => {
                let next = *position as isize + argument;
                *position = usize::try_from(next).expect("jumped to negative position");
            }
            Operation::Nop => *position += 1,
        }
    }
}

impl FromStr for Program {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let instructions = s.lines().map(str::parse).collect::<Result<Vec<_>>>()?;

        Ok(Self {
            instructions,
            position: 0,
            accumulator: 0,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Instruction {
    operation: Operation,
    argument: isize,
}

impl Instruction {
    fn swapped(self) -> Option<Self> {
        match self.operation {
            Operation::Acc => None,
            Operation::Jmp => Some(Self {
                operation: Operation::Nop,
                ..self
            }),
            Operation::Nop => Some(Self {
                operation: Operation::Jmp,
                ..self
            }),
        }
    }
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (operation, argument) = s
            .trim()
            .split_once(' ')
            .ok_or_else(|| anyhow!("malformed instruction: {s}"))?;

        let operation: Operation = operation.parse()?;
        let argument: isize = argument.parse()?;

        Ok(Self {
            operation,
            argument,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Acc,
    Jmp,
    Nop,
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "acc" => Ok(Self::Acc),
            "jmp" => Ok(Self::Jmp),
            "nop" => Ok(Self::Nop),
            _ => bail!("unknown operation: {s}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn test_fixed_accumulator() {
        let result = EXAMPLE
            .parse::<Program>()
            .unwrap()
            .accumulator_after_fix()
            .unwrap();
        assert_eq!(result, 8);
    }

    #[test]
    fn test_run_until_repeat() {
        let mut program = EXAMPLE.parse::<Program>().unwrap();
        let result = program.run();
        assert_eq!(result, RunResult::Repeated);
        assert_eq!(program.accumulator, 5);
    }

    #[test]
    fn test_parse_instruction() {
        let instruction = "jmp -4".parse::<Instruction>().unwrap();
        assert_eq!(
            instruction,
            Instruction {
                operation: Operation::Jmp,
                argument: -4,
            }
        );
    }
}

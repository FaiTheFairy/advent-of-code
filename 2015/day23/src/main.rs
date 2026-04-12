use std::{fs, str::FromStr};

use anyhow::{Context, Result, bail, ensure};

fn main() -> Result<()> {
    let machine: Machine = fs::read_to_string("input.txt")?.parse()?;

    let mut part_1 = machine.clone();
    part_1.run();
    let sol1 = part_1.read(RegisterId::B);
    println!("Part 1: {sol1}");

    let mut part_2 = machine.clone();
    part_2.a = 1;
    part_2.run();
    let sol2 = part_2.read(RegisterId::B);
    println!("Part 2: {sol2}");

    Ok(())
}

type Offset = isize;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Machine {
    a: usize,
    b: usize,
    pointer: usize,
    instructions: Vec<Instruction>,
}

impl Machine {
    fn run(&mut self) {
        while self.step() {}
    }

    fn read(&self, reg: RegisterId) -> usize {
        match reg {
            RegisterId::A => self.a,
            RegisterId::B => self.b,
        }
    }

    fn write(&mut self, reg: RegisterId, value: usize) {
        match reg {
            RegisterId::A => self.a = value,
            RegisterId::B => self.b = value,
        }
    }

    fn step(&mut self) -> bool {
        let Some(instruction) = self.instructions.get(self.pointer).copied() else {
            return false;
        };

        match instruction {
            Instruction::Half(reg) => {
                let value = self.read(reg);
                self.write(reg, value / 2);
                self.pointer += 1;
            }
            Instruction::Triple(reg) => {
                let value = self.read(reg);
                self.write(reg, value * 3);
                self.pointer += 1;
            }
            Instruction::Increment(reg) => {
                let value = self.read(reg);
                self.write(reg, value + 1);
                self.pointer += 1
            }
            Instruction::Jump(offset) => {
                self.pointer = self.pointer.strict_add_signed(offset);
            }
            Instruction::JumpIfEven(reg, offset) => {
                if self.read(reg).is_multiple_of(2) {
                    self.pointer = self.pointer.strict_add_signed(offset);
                } else {
                    self.pointer += 1;
                }
            }
            Instruction::JumpIfOne(reg, offset) => {
                if self.read(reg) == 1 {
                    self.pointer = self.pointer.strict_add_signed(offset);
                } else {
                    self.pointer += 1;
                }
            }
        }

        true
    }
}

impl FromStr for Machine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions: Vec<Instruction> = s
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        Ok(Self {
            instructions,
            ..Default::default()
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum RegisterId {
    A,
    B,
}

impl FromStr for RegisterId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            _ => bail!("unknown register id: {s}"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Instruction {
    Half(RegisterId),
    Triple(RegisterId),
    Increment(RegisterId),
    Jump(Offset),
    JumpIfEven(RegisterId, Offset),
    JumpIfOne(RegisterId, Offset),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instruction, args) = s
            .trim()
            .split_once(' ')
            .context("missing space separator")?;

        let args: Vec<&str> = args
            .split_whitespace()
            .map(|s| s.trim_end_matches(','))
            .collect();

        match instruction {
            "hlf" => {
                ensure!(args.len() == 1, "hlf only takes 1 argument");
                Ok(Self::Half(args[0].parse()?))
            }
            "tpl" => {
                ensure!(args.len() == 1, "tpl only takes 1 argument");
                Ok(Self::Triple(args[0].parse()?))
            }
            "inc" => {
                ensure!(args.len() == 1, "inc only takes 1 argument");
                Ok(Self::Increment(args[0].parse()?))
            }
            "jmp" => {
                ensure!(args.len() == 1, "jmp only takes 1 argument");
                Ok(Self::Jump(args[0].parse()?))
            }
            "jie" => {
                ensure!(args.len() == 2, "jie takes exactly 2 arguments");
                Ok(Self::JumpIfEven(args[0].parse()?, args[1].parse()?))
            }
            "jio" => {
                ensure!(args.len() == 2, "jio takes exactly 2 arguments");
                Ok(Self::JumpIfOne(args[0].parse()?, args[1].parse()?))
            }
            _ => bail!("unknown instruction: {s}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example_program() {
        let instructions: Vec<Instruction> = vec![
            "inc a".parse().unwrap(),
            "jio a, +2".parse().unwrap(),
            "tpl a".parse().unwrap(),
            "inc a".parse().unwrap(),
        ];

        let mut machine: Machine = Machine {
            a: 0,
            b: 0,
            pointer: 0,
            instructions,
        };

        machine.run();

        assert_eq!(machine.a, 2);
        assert_eq!(machine.b, 0);
    }
    #[test]
    fn test_parse_instruction() {
        assert_eq!(
            "jio a, +2".parse::<Instruction>().unwrap(),
            Instruction::JumpIfOne(RegisterId::A, 2)
        );

        assert_eq!(
            "tpl a".parse::<Instruction>().unwrap(),
            Instruction::Triple(RegisterId::A)
        );

        assert_eq!(
            "inc a".parse::<Instruction>().unwrap(),
            Instruction::Increment(RegisterId::A)
        );

        assert_eq!(
            "jio b, +2".parse::<Instruction>().unwrap(),
            Instruction::JumpIfOne(RegisterId::B, 2)
        );
    }
}

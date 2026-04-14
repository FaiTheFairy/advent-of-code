#![allow(dead_code)]

use std::{fs, str::FromStr};

use anyhow::{Context, Result, bail, ensure};

fn main() -> Result<()> {
    let machine: Machine = fs::read_to_string("input.txt")?.parse()?;

    let mut machine_1 = machine.clone();
    machine_1.run();
    let sol1 = machine_1.read_register(Register::A);
    println!("Part 1: {sol1}");

    let mut machine_2 = machine.clone();
    machine_2.write_register(Register::C, 1);
    machine_2.run();
    let sol2 = machine_2.read_register(Register::A);
    println!("Part 2: {sol2}");

    Ok(())
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Machine {
    /// Program counter.
    pc: i64,
    registers: Registers,
    program: Vec<Instruction>,
}

impl Machine {
    fn run(&mut self) {
        while self.step() {}
    }

    fn step(&mut self) -> bool {
        if self.pc < 0 || self.pc >= self.program.len() as i64 {
            return false;
        }

        match self.program[self.pc as usize] {
            Instruction::Copy { from, to } => {
                let value = self.eval(from);
                self.write_register(to, value);
                self.pc += 1;
            }
            Instruction::Increment(reg) => {
                let value = self.read_register(reg);
                self.write_register(reg, value + 1);
                self.pc += 1;
            }
            Instruction::Decrement(reg) => {
                let value = self.read_register(reg);
                self.write_register(reg, value - 1);
                self.pc += 1;
            }
            Instruction::Jump { test, offset } => {
                if self.eval(test) != 0 {
                    self.pc += self.eval(offset);
                } else {
                    self.pc += 1;
                }
            }
        }
        true
    }

    fn read_register(&self, reg: Register) -> i64 {
        self.registers.read(reg)
    }

    fn write_register(&mut self, reg: Register, value: i64) {
        self.registers.write(reg, value);
    }

    fn eval(&self, operand: Operand) -> i64 {
        match operand {
            Operand::Register(reg) => self.read_register(reg),
            Operand::Value(value) => value,
        }
    }
}

impl FromStr for Machine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let program = s
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        Ok(Self {
            program,
            ..Default::default()
        })
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
struct Registers {
    a: i64,
    b: i64,
    c: i64,
    d: i64,
}

impl Registers {
    fn read(&self, reg: Register) -> i64 {
        match reg {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
        }
    }

    fn write(&mut self, reg: Register, value: i64) {
        match reg {
            Register::A => self.a = value,
            Register::B => self.b = value,
            Register::C => self.c = value,
            Register::D => self.d = value,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Register {
    A,
    B,
    C,
    D,
}

impl FromStr for Register {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "a" => Ok(Register::A),
            "b" => Ok(Register::B),
            "c" => Ok(Register::C),
            "d" => Ok(Register::D),
            _ => bail!("invalid register: {s}"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Operand {
    Register(Register),
    Value(i64),
}

impl FromStr for Operand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if let Ok(reg) = s.parse() {
            return Ok(Operand::Register(reg));
        }

        let value = s.parse().with_context(|| format!("invalid operand: {s}"))?;

        Ok(Operand::Value(value))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Instruction {
    Copy { from: Operand, to: Register },
    Increment(Register),
    Decrement(Register),
    Jump { test: Operand, offset: Operand },
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        let op = parts.next().context("missing opcode")?;

        let out = match op {
            "cpy" => {
                let from = parts.next().context("missing source")?.parse()?;
                let to = parts.next().context("missing destination")?.parse()?;

                Ok(Instruction::Copy { from, to })
            }
            "inc" => {
                let reg = parts.next().context("missing register")?.parse()?;
                Ok(Instruction::Increment(reg))
            }
            "dec" => {
                let reg = parts.next().context("missing register")?.parse()?;
                Ok(Instruction::Decrement(reg))
            }
            "jnz" => {
                let test = parts.next().context("missing test")?.parse()?;
                let offset = parts.next().context("missing offset")?.parse()?;

                Ok(Instruction::Jump { test, offset })
            }
            _ => bail!("unknown instruction: {op}"),
        };

        ensure!(
            parts.next().is_none(),
            "instruction has too many arguments passed"
        );
        out
    }
}

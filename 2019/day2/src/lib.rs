#![allow(unused)]

use std::str::FromStr;

use anyhow::{Context, Result, anyhow, bail};

pub fn solve_part_1(input: &str) -> Result<isize> {
    let program: Program = input.parse()?;
    let mut vm = program.to_machine();
    vm.write(Address(1), Value(12))?;
    vm.write(Address(2), Value(2))?;
    vm.run()?;
    vm.read(Address(0)).map(|v| v.0)
}

pub fn solve_part_2(input: &str) -> Result<isize> {
    let program: Program = input.parse()?;
    for noun in 0..100 {
        for verb in 0..100 {
            let mut vm = program.to_machine();
            vm.write(Address(1), Value(noun))?;
            vm.write(Address(2), Value(verb))?;
            vm.run()?;
            if vm.read(Address(0))? == Value(19690720) {
                return Ok(100 * noun + verb);
            }
        }
    }
    bail!("couldn't find pair of noun and verb within 0..=99 that yield the desired value")
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Program {
    memory: Vec<Value>,
}

impl Program {
    fn to_machine(&self) -> Machine {
        Machine {
            memory: self.memory.clone(),
            pointer: Address(0),
        }
    }
}

impl FromStr for Program {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let memory = s
            .trim()
            .split(',')
            .map(str::trim)
            .map(str::parse::<isize>)
            .map(|r| r.map(Value))
            .collect::<Result<_, _>>()
            .context("failed to parse comma-separated integers")?;

        Ok(Self { memory })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Machine {
    memory: Vec<Value>,
    pointer: Address,
}

impl Machine {
    fn run(&mut self) -> Result<()> {
        loop {
            match self.decode()? {
                DecodedInstr::Add(bin_op_instr) => {
                    let BinOpInstr {
                        lhs_addr: lhs,
                        rhs_addr: rhs,
                        dst_addr: dst,
                    } = bin_op_instr;
                    let lhs = self.read(lhs)?;
                    let rhs = self.read(rhs)?;
                    self.write(dst, Value(lhs.0 + rhs.0))?;
                    self.advance(4)?;
                }
                DecodedInstr::Mul(bin_op_instr) => {
                    let BinOpInstr {
                        lhs_addr: lhs,
                        rhs_addr: rhs,
                        dst_addr: dst,
                    } = bin_op_instr;
                    let lhs = self.read(lhs)?;
                    let rhs = self.read(rhs)?;
                    self.write(dst, Value(lhs.0 * rhs.0))?;
                    self.advance(4)?;
                }
                DecodedInstr::Halt => break,
            }
        }

        Ok(())
    }

    fn advance(&mut self, amount: usize) -> Result<()> {
        self.pointer = self
            .pointer
            .0
            .checked_add(amount)
            .map(Address)
            .ok_or(anyhow!("instruction pointer overflow"))?;

        Ok(())
    }

    fn write(&mut self, addr: Address, value: Value) -> Result<()> {
        let slot = self
            .memory
            .get_mut(addr.0)
            .ok_or_else(|| anyhow!("address out of bounds: {addr:?}"))?;

        *slot = value;
        Ok(())
    }

    fn read(&self, addr: Address) -> Result<Value> {
        let word = self
            .memory
            .get(addr.0)
            .ok_or_else(|| anyhow!("No word found for address {}", addr.0))?;

        Ok(*word)
    }

    fn decode(&self) -> Result<DecodedInstr> {
        let opcode = Opcode::try_from(self.read(self.pointer)?)?;

        match opcode {
            Opcode::Add => Ok(DecodedInstr::Add(BinOpInstr {
                lhs_addr: Address(usize::try_from(self.read(self.pointer_offset(1)?)?.0)?),
                rhs_addr: Address(usize::try_from(self.read(self.pointer_offset(2)?)?.0)?),
                dst_addr: Address(usize::try_from(self.read(self.pointer_offset(3)?)?.0)?),
            })),
            Opcode::Mul => Ok(DecodedInstr::Mul(BinOpInstr {
                lhs_addr: Address(usize::try_from(self.read(self.pointer_offset(1)?)?.0)?),
                rhs_addr: Address(usize::try_from(self.read(self.pointer_offset(2)?)?.0)?),
                dst_addr: Address(usize::try_from(self.read(self.pointer_offset(3)?)?.0)?),
            })),
            Opcode::Halt => Ok(DecodedInstr::Halt),
        }
    }

    fn pointer_offset(&self, offset: usize) -> Result<Address> {
        self.pointer
            .0
            .checked_add(offset)
            .map(Address)
            .ok_or(anyhow!("overflow in pointer offset"))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Value(isize);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Address(usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Opcode {
    Add,
    Mul,
    Halt,
}

impl TryFrom<Value> for Opcode {
    type Error = anyhow::Error;

    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        match value.0 {
            1 => Ok(Self::Add),
            2 => Ok(Self::Mul),
            99 => Ok(Self::Halt),
            other => bail!("unknown opcode: {other}"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct BinOpInstr {
    lhs_addr: Address,
    rhs_addr: Address,
    dst_addr: Address,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum DecodedInstr {
    Add(BinOpInstr),
    Mul(BinOpInstr),
    Halt,
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "1,9,10,3,2,3,11,0,99,30,40,50";
}

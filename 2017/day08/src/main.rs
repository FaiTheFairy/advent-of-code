use std::{collections::HashMap, fs, str::FromStr};

use anyhow::{Context, Result, bail};
use derive_more::{Add, AddAssign, Display, From, Sub, SubAssign};

fn main() -> Result<()> {
    let input: Input = fs::read_to_string("input.txt")?.parse()?;

    let mut cpu = Cpu::default();
    cpu.apply_instructions(&input.0);

    let sol1 = cpu.registers.values().max().copied().unwrap_or_default();
    println!("Part 1: {sol1}");

    let sol2 = cpu.abs_max;
    println!("Part 2: {sol2}");

    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Input(Vec<Instruction>);

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let inner = s
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        Ok(Self(inner))
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Cpu {
    registers: HashMap<RegisterName, Value>,
    abs_max: Value,
}

impl Cpu {
    fn apply_instructions(&mut self, instructions: &[Instruction]) {
        for instruction in instructions {
            self.apply_instruction(instruction);
        }
    }

    fn apply_instruction(&mut self, instruction: &Instruction) {
        if self.check_condition(&instruction.condition) {
            let val = self.apply_action(instruction.register.clone(), instruction.action);
            self.abs_max = self.abs_max.max(val);
        }
    }

    fn apply_action(&mut self, register: RegisterName, action: Action) -> Value {
        let entry = self.registers.entry(register).or_insert(Value(0));

        match action {
            Action::Increment(value) => *entry += value,
            Action::Decrement(value) => *entry -= value,
        }

        *entry
    }

    fn check_condition(&self, condition: &Condition) -> bool {
        let Condition { lhs, operator, rhs } = condition;
        let lhs = self.registers.get(lhs).copied().unwrap_or_default();
        operator.eval(lhs, *rhs)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, From, Display)]
struct RegisterName(String);

#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Display,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Add,
    AddAssign,
    Sub,
    SubAssign,
)]
struct Value(i32);

#[derive(Clone, Debug, PartialEq, Eq)]
struct Instruction {
    register: RegisterName,
    action: Action,
    condition: Condition,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (register, rest) = s.trim().split_once(' ').context("empty instruction")?;
        let register = RegisterName(register.to_owned());

        let (action, condition) = rest
            .split_once("if")
            .context("instruction missing conditional `if`")?;

        let action = action.parse()?;
        let condition = condition.parse()?;

        Ok(Self {
            register,
            action,
            condition,
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Action {
    Increment(Value),
    Decrement(Value),
}

impl FromStr for Action {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        let action = iter.next().context("empty action")?;
        let value = Value(iter.next().context("empty action value")?.parse()?);
        match action {
            "inc" => Ok(Self::Increment(value)),
            "dec" => Ok(Self::Decrement(value)),
            _ => bail!("unknown action: {s}"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Condition {
    lhs: RegisterName,
    operator: ComparisonOperator,
    rhs: Value,
}

impl FromStr for Condition {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        let lhs = RegisterName(
            iter.next()
                .context("condition missing register (lhs)")?
                .to_owned(),
        );
        let operator = iter
            .next()
            .context("condition missing comparison operator")?
            .parse()?;
        let rhs = Value(
            iter.next()
                .context("comparison missing value (rhs)")?
                .parse()?,
        );

        Ok(Self { lhs, operator, rhs })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum ComparisonOperator {
    Greater,
    Equal,
    Less,
    GreaterEqual,
    LessEqual,
    NotEqual,
}
impl ComparisonOperator {
    fn eval(self, lhs: Value, rhs: Value) -> bool {
        match self {
            ComparisonOperator::Greater => lhs > rhs,
            ComparisonOperator::Equal => lhs == rhs,
            ComparisonOperator::Less => lhs < rhs,
            ComparisonOperator::GreaterEqual => lhs >= rhs,
            ComparisonOperator::LessEqual => lhs <= rhs,
            ComparisonOperator::NotEqual => lhs != rhs,
        }
    }
}

impl FromStr for ComparisonOperator {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.trim() {
            ">" => Ok(Self::Greater),
            "==" => Ok(Self::Equal),
            "<" => Ok(Self::Less),
            ">=" => Ok(Self::GreaterEqual),
            "<=" => Ok(Self::LessEqual),
            "!=" => Ok(Self::NotEqual),
            _ => bail!("unknown comparison operator: {s}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";

    #[test]
    fn test_apply_instructions_and_max() {
        let mut cpu = Cpu::default();
        let input: Input = EXAMPLE.parse().unwrap();
        cpu.apply_instructions(&input.0);
        assert_eq!(cpu.registers.values().max().unwrap(), &Value(1));
        assert_eq!(cpu.abs_max, Value(10));
    }

    #[test]
    fn test_apply_instruction() {
        let instruction: Instruction = "a inc 1 if b < 5".parse().unwrap();
        let mut cpu = Cpu::default();
        cpu.apply_instruction(&instruction);
        assert_eq!(
            cpu.registers.get(&RegisterName("a".into())).unwrap(),
            &Value(1)
        );
    }

    #[test]
    fn test_parse_instruction() {
        let result: Instruction = "b inc 5 if a > 1".parse().unwrap();
        let expected = Instruction {
            register: RegisterName("b".to_owned()),
            action: Action::Increment(Value(5)),
            condition: Condition {
                lhs: RegisterName("a".into()),
                operator: ComparisonOperator::Greater,
                rhs: Value(1),
            },
        };
        assert_eq!(result, expected);
    }
}

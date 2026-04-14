use std::{collections::HashMap, fs, str::FromStr};

use anyhow::{Context, Result, bail};

fn main() -> Result<()> {
    let input: Input = fs::read_to_string("input.txt")
        .context("failed to read input.txt")?
        .parse()
        .context("failed to parse input")?;

    let mut factory_1: Factory = Factory::default();
    factory_1.apply_instructions(input.instructions());
    let sol1: BotId = factory_1
        .find_comparer(Value(17), Value(61))
        .context("no solution found for part 1")?;
    println!("Part 1: {sol1}");

    let mut factory_2: Factory = Factory::default();
    factory_2.apply_instructions(input.instructions());
    factory_2.run_to_completion();
    let sol2: usize = factory_2
        .output_product_0_1_2()
        .context("missing value in output 0, 1, or 2")?;
    println!("Part 2: {sol2}");

    Ok(())
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Value(usize);

impl Value {
    fn get(self) -> usize {
        self.0
    }
}

impl FromStr for Value {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct BotId(usize);

impl FromStr for BotId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

impl std::fmt::Display for BotId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct OutputId(usize);

impl FromStr for OutputId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Destination {
    Bot(BotId),
    Output(OutputId),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Rule {
    low: Destination,
    high: Destination,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Bot {
    chips: Vec<Value>,
}

impl Bot {
    fn push(&mut self, value: Value) {
        debug_assert!(self.chips.len() < 2);
        self.chips.push(value);
    }

    fn is_ready(&self) -> bool {
        self.chips.len() == 2
    }

    fn low_high(&self) -> Option<(Value, Value)> {
        match self.chips.as_slice() {
            [a, b] if a <= b => Some((*a, *b)),
            [a, b] => Some((*b, *a)),
            _ => None,
        }
    }

    fn is_comparing(&self, a: Value, b: Value) -> bool {
        let low: Value = if a <= b { a } else { b };
        let high: Value = if a <= b { b } else { a };
        self.low_high() == Some((low, high))
    }

    fn clear(&mut self) {
        self.chips.clear();
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Output {
    values: Vec<Value>,
}

impl Output {
    fn push(&mut self, value: Value) {
        self.values.push(value);
    }

    fn first(&self) -> Option<Value> {
        self.values.first().copied()
    }
}

#[derive(Clone, Debug, Default)]
struct Factory {
    bots: HashMap<BotId, Bot>,
    rules: HashMap<BotId, Rule>,
    outputs: HashMap<OutputId, Output>,
}

impl Factory {
    fn apply_instructions(&mut self, instructions: &[Instruction]) {
        for &instruction in instructions {
            self.apply_instruction(instruction);
        }
    }

    fn apply_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::InitialValue { value, bot } => {
                self.bots.entry(bot).or_default().push(value);
            }
            Instruction::GiveRule { bot, rule } => {
                self.rules.insert(bot, rule);
            }
        }
    }

    fn give_to_destination(&mut self, destination: Destination, value: Value) {
        match destination {
            Destination::Bot(bot_id) => {
                self.bots.entry(bot_id).or_default().push(value);
            }
            Destination::Output(output_id) => {
                self.outputs.entry(output_id).or_default().push(value);
            }
        }
    }

    fn step(&mut self) -> Option<BotId> {
        let bot_id: BotId = self
            .bots
            .iter()
            .find_map(|(&bot_id, bot)| bot.is_ready().then_some(bot_id))?;

        let (low_value, high_value): (Value, Value) = self.bots.get(&bot_id)?.low_high()?;
        let rule: Rule = *self.rules.get(&bot_id)?;

        self.bots.get_mut(&bot_id)?.clear();
        self.give_to_destination(rule.low, low_value);
        self.give_to_destination(rule.high, high_value);

        Some(bot_id)
    }

    fn find_comparer(&mut self, a: Value, b: Value) -> Option<BotId> {
        loop {
            for (&bot_id, bot) in &self.bots {
                if bot.is_comparing(a, b) {
                    return Some(bot_id);
                }
            }

            self.step()?;
        }
    }

    fn run_to_completion(&mut self) {
        while self.step().is_some() {}
    }

    fn output_product_0_1_2(&self) -> Option<usize> {
        let a: usize = self.outputs.get(&OutputId(0))?.first()?.get();
        let b: usize = self.outputs.get(&OutputId(1))?.first()?.get();
        let c: usize = self.outputs.get(&OutputId(2))?.first()?.get();
        Some(a * b * c)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Input(Vec<Instruction>);

impl Input {
    fn instructions(&self) -> &[Instruction] {
        &self.0
    }
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions: Vec<Instruction> = s
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        Ok(Self(instructions))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Instruction {
    InitialValue { value: Value, bot: BotId },
    GiveRule { bot: BotId, rule: Rule },
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split_whitespace().collect();

        match tokens.as_slice() {
            ["value", value, "goes", "to", "bot", bot] => {
                let value: Value = value.parse()?;
                let bot: BotId = bot.parse()?;
                Ok(Self::InitialValue { value, bot })
            }
            [
                "bot",
                bot,
                "gives",
                "low",
                "to",
                low_kind,
                low_id,
                "and",
                "high",
                "to",
                high_kind,
                high_id,
            ] => {
                let bot: BotId = bot.parse()?;
                let low: Destination = parse_destination(low_kind, low_id)?;
                let high: Destination = parse_destination(high_kind, high_id)?;
                Ok(Self::GiveRule {
                    bot,
                    rule: Rule { low, high },
                })
            }
            _ => bail!("unknown instruction: {s}"),
        }
    }
}

fn parse_destination(kind: &str, id: &str) -> Result<Destination> {
    match kind {
        "bot" => Ok(Destination::Bot(id.parse()?)),
        "output" => Ok(Destination::Output(id.parse()?)),
        _ => bail!("unknown destination kind: {kind}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2
";

    #[test]
    fn test_parse_initial_value_instruction() {
        let instruction: Instruction = "value 5 goes to bot 2".parse().unwrap();
        assert_eq!(
            instruction,
            Instruction::InitialValue {
                value: Value(5),
                bot: BotId(2),
            }
        );
    }

    #[test]
    fn test_parse_rule_instruction() {
        let instruction: Instruction = "bot 2 gives low to bot 1 and high to output 0"
            .parse()
            .unwrap();

        assert_eq!(
            instruction,
            Instruction::GiveRule {
                bot: BotId(2),
                rule: Rule {
                    low: Destination::Bot(BotId(1)),
                    high: Destination::Output(OutputId(0)),
                },
            }
        );
    }

    #[test]
    fn test_example_part_1() {
        let input: Input = EXAMPLE.parse().unwrap();
        let mut factory: Factory = Factory::default();
        factory.apply_instructions(input.instructions());

        let comparer: BotId = factory.find_comparer(Value(2), Value(5)).unwrap();
        assert_eq!(comparer, BotId(2));
    }

    #[test]
    fn test_example_part_2() {
        let input: Input = EXAMPLE.parse().unwrap();
        let mut factory: Factory = Factory::default();
        factory.apply_instructions(input.instructions());
        factory.run_to_completion();

        assert_eq!(
            factory.outputs.get(&OutputId(0)).unwrap().first(),
            Some(Value(5))
        );
        assert_eq!(
            factory.outputs.get(&OutputId(1)).unwrap().first(),
            Some(Value(2))
        );
        assert_eq!(
            factory.outputs.get(&OutputId(2)).unwrap().first(),
            Some(Value(3))
        );
        assert_eq!(factory.output_product_0_1_2(), Some(30));
    }
}

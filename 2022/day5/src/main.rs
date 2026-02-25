use std::{fs, str::FromStr};

use anyhow::{Context, Result, bail, ensure};

fn main() -> Result<()> {
    let input = fs::read_to_string("./input.txt").context("failed to read ./input.txt")?;
    let problem = input.parse::<ProblemInput>()?;

    // Part 1: CrateMover 9000 (moves one-by-one; reverses order)
    let mut stacks1 = problem.stacks.clone();
    stacks1.apply_all(&problem.instructions, CraneMode::CrateMover9000)?;
    println!("Part 1. top crates message = {}", stacks1.top_message()?);

    // Part 2: CrateMover 9001 (moves as a block; preserves order)
    let mut stacks2 = problem.stacks.clone();
    stacks2.apply_all(&problem.instructions, CraneMode::CrateMover9001)?;
    println!("Part 2. top crates message = {}", stacks2.top_message()?);

    Ok(())
}

#[derive(Debug, Clone)]
struct ProblemInput {
    stacks: Stacks,
    instructions: Instructions,
}

impl FromStr for ProblemInput {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s1, s2) = s
            .split_once("\n\n")
            .context("couldn't find empty line separating stacks from instructions")?;

        let stacks = s1.parse::<Stacks>().context("failed to parse stacks")?;
        let instructions = s2
            .parse::<Instructions>()
            .context("failed to parse instructions")?;

        Ok(Self {
            stacks,
            instructions,
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum CraneMode {
    CrateMover9000,
    CrateMover9001,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Stacks(Vec<Vec<CrateLabel>>);

impl Stacks {
    fn apply_all(&mut self, instructions: &Instructions, mode: CraneMode) -> Result<()> {
        for mv in &instructions.0 {
            self.apply(*mv, mode)?;
        }
        Ok(())
    }

    fn apply(&mut self, mv: MoveCrates, mode: CraneMode) -> Result<()> {
        let MoveCrates { n, from, to } = mv;

        ensure!(
            from < self.0.len(),
            "from stack index out of bounds: {from}",
        );
        ensure!(to < self.0.len(), "to stack index out of bounds: {to}");
        ensure!(from != to, "from and to stacks are the same: {from}");

        let available = self.0[from].len();
        ensure!(
            n <= available,
            "attempted to move {n} crates from stack {from}, but only {available} available"
        );

        match mode {
            CraneMode::CrateMover9000 => {
                for _ in 0..n {
                    let c = self.0[from]
                        .pop()
                        .context("internal error: pop failed despite length check")?;
                    self.0[to].push(c);
                }
            }
            CraneMode::CrateMover9001 => {
                let len = self.0[from].len();
                let moved = self.0[from].split_off(len - n);
                self.0[to].extend(moved);
            }
        }

        Ok(())
    }

    fn top_message(&self) -> Result<String> {
        let mut out = String::with_capacity(self.0.len());
        for (i, stack) in self.0.iter().enumerate() {
            let top = stack
                .last()
                .with_context(|| format!("stack {i} is empty; cannot read top crate"))?;
            out.push(top.as_char());
        }
        Ok(out)
    }
}

impl FromStr for Stacks {
    type Err = anyhow::Error;

    /// Parses the "drawing", e.g.
    /// ```text
    ///     [D]
    /// [N] [C]
    /// [Z] [M] [P]
    ///  1   2   3
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines: Vec<&str> = s.lines().collect();

        let numbers = lines
            .pop()
            .context("missing stack number line at the end of stacks input")?;

        let stack_count = numbers.split_whitespace().count();
        ensure!(stack_count != 0, "stack count cannot be zero");

        let mut stacks: Vec<Vec<CrateLabel>> = vec![Vec::new(); stack_count];

        // parse from bottom up so that the top crate ends up at the end of the Vec
        for line in lines.iter().rev() {
            for (i, stack) in stacks.iter_mut().enumerate() {
                let col = 1 + i * 4; // AoC diagram spacing convention

                if let Some(b) = line.as_bytes().get(col).copied()
                    && b.is_ascii_uppercase()
                {
                    stack.push(CrateLabel::try_new(b)?);
                }
            }
        }

        Ok(Self(stacks))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CrateLabel(u8);

impl CrateLabel {
    fn try_new(label: u8) -> Result<Self> {
        if label.is_ascii_uppercase() {
            Ok(Self(label))
        } else {
            bail!("crate label must be an uppercase ASCII character, got byte={label}")
        }
    }

    fn as_char(self) -> char {
        self.0 as char
    }
}

#[derive(Debug, Clone)]
struct Instructions(Vec<MoveCrates>);

impl FromStr for Instructions {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let moves = s
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(str::parse::<MoveCrates>)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self(moves))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MoveCrates {
    n: usize,
    from: usize, // 0-based
    to: usize,   // 0-based
}

impl FromStr for MoveCrates {
    type Err = anyhow::Error;

    /// Parses `"move 1 from 2 to 1"`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        ensure!(parts.len() == 6, "input is malformed: '{s}'");
        ensure!(parts[0] == "move", "expected 'move', got '{}'", parts[0]);
        ensure!(parts[2] == "from", "expected 'from', got '{}'", parts[2]);
        ensure!(parts[4] == "to", "expected 'to', got '{}'", parts[4]);

        let n = parts[1]
            .parse::<usize>()
            .context("failed to parse move count")?;

        // convert to 0-based indices right here
        let from_1 = parts[3]
            .parse::<usize>()
            .context("failed to parse from index")?;
        let to_1 = parts[5]
            .parse::<usize>()
            .context("failed to parse to index")?;
        ensure!(from_1 != 0 && to_1 != 0, "stack indices must start at 1");

        Ok(Self {
            n,
            from: from_1 - 1,
            to: to_1 - 1,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_parse_stacks() -> Result<()> {
        let input = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3";
        let result = input.parse::<Stacks>()?;

        let expected = Stacks(vec![
            vec![CrateLabel::try_new(b'Z')?, CrateLabel::try_new(b'N')?],
            vec![
                CrateLabel::try_new(b'M')?,
                CrateLabel::try_new(b'C')?,
                CrateLabel::try_new(b'D')?,
            ],
            vec![CrateLabel::try_new(b'P')?],
        ]);

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_example_part_1() -> Result<()> {
        let problem = EXAMPLE.parse::<ProblemInput>()?;
        let mut stacks = problem.stacks.clone();
        stacks.apply_all(&problem.instructions, CraneMode::CrateMover9000)?;
        assert_eq!(stacks.top_message()?, "CMZ");
        Ok(())
    }

    #[test]
    fn test_example_part_2() -> Result<()> {
        let problem = EXAMPLE.parse::<ProblemInput>()?;
        let mut stacks = problem.stacks.clone();
        stacks.apply_all(&problem.instructions, CraneMode::CrateMover9001)?;
        assert_eq!(stacks.top_message()?, "MCD");
        Ok(())
    }
}

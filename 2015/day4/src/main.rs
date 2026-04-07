use std::time::{Duration, SystemTime};

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let input = "iwrupvqb";

    let now = SystemTime::now();
    let sol1 = (0..1_000_000_000)
        .find(|n| {
            Input {
                key: input.into(),
                answer: *n,
            }
            .satisfies_part_1()
        })
        .context("no solution found for part 1")?;
    let elapsed = now.elapsed()?;
    println!("Part 1: {sol1}");
    println!("elapsed: {elapsed:?}");

    let now = SystemTime::now();
    let sol2 = (0..1_000_000_000)
        .find(|n| {
            Input {
                key: input.into(),
                answer: *n,
            }
            .satisfies_part_2()
        })
        .context("no solution found for part 2")?;

    let elapsed = now.elapsed()?;
    println!("Part 2: {sol2}");
    println!("elapsed: {elapsed:?}");

    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Input {
    key: String,
    answer: usize,
}

impl Input {
    fn satisfies_part_1(&self) -> bool {
        self.md5().starts_with("00000")
    }

    fn satisfies_part_2(&self) -> bool {
        self.md5().starts_with("000000")
    }

    fn md5(&self) -> String {
        let input = format!("{}{}", self.key, self.answer);
        let digest = md5::compute(input.as_bytes());
        format!("{digest:x}")
    }
}

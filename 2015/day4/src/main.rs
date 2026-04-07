use std::time::SystemTime;

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
        let digest = self.md5();
        // we are looking for first 5 digits in the hexadecimal representation
        // to be 0. a u8 is represented by two hex digits.
        // [ high 4 bits ][ low 4 bits]
        // and the fifth hexademical we want should start be "0?"
        digest[0] == 0 && digest[1] == 0 && digest[2] < 0b0001_0000
    }

    fn satisfies_part_2(&self) -> bool {
        let digest = self.md5();
        digest[0] == 0 && digest[1] == 0 && digest[2] == 0
    }

    fn md5(&self) -> [u8; 16] {
        let input = format!("{}{}", self.key, self.answer);
        md5::compute(input.as_bytes()).0
    }
}

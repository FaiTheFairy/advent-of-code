use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let offsets: Offsets = fs::read_to_string("input.txt")?
        .lines()
        .map(str::parse)
        .collect::<Result<_, _>>()?;

    let cpu = Cpu::new(offsets);

    let sol1 = cpu.clone().run_v1();
    println!("Part 1: {sol1}");

    let sol2 = cpu.clone().run_v2();
    println!("Part 2: {sol2}");

    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Cpu {
    pointer: i32,
    offsets: Offsets,
}

impl Cpu {
    fn run_v1(&mut self) -> usize {
        let mut steps = 0;
        while (0..self.offsets.len() as i32).contains(&self.pointer) {
            let offset = self.offsets[self.pointer as usize];
            self.offsets[self.pointer as usize] += 1;
            self.pointer += offset;
            steps += 1;
        }
        steps
    }

    fn run_v2(&mut self) -> usize {
        let mut steps = 0;
        while (0..self.offsets.len() as i32).contains(&self.pointer) {
            let offset = self.offsets[self.pointer as usize];
            self.offsets[self.pointer as usize] += if offset >= 3 { -1 } else { 1 };
            self.pointer += offset;
            steps += 1;
        }
        steps
    }

    fn new(offsets: Offsets) -> Self {
        Self {
            pointer: 0,
            offsets,
        }
    }
}

type Offset = i32;
type Offsets = Vec<Offset>;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "0\n3\n0\n1\n-3";

    fn cpu() -> Cpu {
        Cpu {
            pointer: 0,
            offsets: EXAMPLE
                .lines()
                .map(str::parse)
                .collect::<Result<_, _>>()
                .unwrap(),
        }
    }

    #[test]
    fn test_part_2() {
        assert_eq!(cpu().run_v2(), 10);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(cpu().run_v1(), 5);
    }
}

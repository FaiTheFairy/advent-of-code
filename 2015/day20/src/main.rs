use anyhow::{Context, Result};

const ELF_LIMIT: usize = 1_000_000;

fn main() -> Result<()> {
    let sol1 = lowest_house_number_v1(34_000_000).context("no solution found within limit")?;
    println!("Part 1: {sol1}");

    let sol2 = lowest_house_number_v2(34_000_000).context("no solution found within limit")?;
    println!("Part 2: {sol2}");

    Ok(())
}

fn lowest_house_number_v1(target: usize) -> Option<usize> {
    let mut houses = vec![0; ELF_LIMIT + 1];
    for elf in 1..=ELF_LIMIT {
        for house in (elf..=ELF_LIMIT).step_by(elf) {
            houses[house] += elf * 10;
        }

        if houses[elf] >= target {
            return Some(elf);
        }
    }
    None
}

fn lowest_house_number_v2(target: usize) -> Option<usize> {
    let mut houses = vec![0; ELF_LIMIT + 1];

    for elf in 1..=ELF_LIMIT {
        // now each elf just delivers presents to 50 houses
        for house in (elf..ELF_LIMIT).step_by(elf).take(50) {
            houses[house] += elf * 11;
        }

        if houses[elf] >= target {
            return Some(elf);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lowest_house_number() {
        let result = lowest_house_number_v1(150).unwrap();
        assert_eq!(result, 8);
    }
}

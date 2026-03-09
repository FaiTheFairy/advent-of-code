use anyhow::{Result, ensure};
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    println!("Part 1. power consumption = {}", solve_part_1(&input)?);
    println!("Part 2. life support rating = {}", solve_part_2(&input)?);
    Ok(())
}

fn parse_to_vec(input: &str) -> Result<Vec<&[u8]>> {
    let rows: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();

    ensure!(!rows.is_empty(), "input is empty");

    let width = rows[0].len();
    ensure!(width > 0, "input rows are empty");
    ensure!(
        rows.iter().all(|row| row.len() == width),
        "input rows have inconsistent widths"
    );
    ensure!(
        rows.iter()
            .all(|row| row.iter().all(|&b| matches!(b, b'0' | b'1'))),
        "input contains non-binary characters"
    );

    Ok(rows)
}

fn bits_to_usize(bits: &[u8]) -> Result<usize> {
    Ok(usize::from_str_radix(std::str::from_utf8(bits)?, 2)?)
}

fn solve_part_1(input: &str) -> Result<usize> {
    let rows = parse_to_vec(input)?;
    let width = rows[0].len();

    let mut gamma = 0usize;

    for i in 0..width {
        let ones = rows.iter().filter(|row| row[i] == b'1').count();
        gamma <<= 1;
        if ones * 2 >= rows.len() {
            gamma |= 1;
        }
    }

    let mask = (1usize << width) - 1;
    let epsilon = (!gamma) & mask;

    Ok(gamma * epsilon)
}

fn solve_part_2(input: &str) -> Result<usize> {
    let rows = parse_to_vec(input)?;
    let oxygen = rating(rows.clone(), true);
    let co2 = rating(rows, false);
    Ok(bits_to_usize(oxygen)? * bits_to_usize(co2)?)
}

fn rating(mut rows: Vec<&[u8]>, keep_most_common: bool) -> &[u8] {
    let width = rows[0].len();

    for i in 0..width {
        if rows.len() == 1 {
            break;
        }

        let ones = rows.iter().filter(|row| row[i] == b'1').count();
        let zeros = rows.len() - ones;

        let keep = if keep_most_common {
            if ones >= zeros { b'1' } else { b'0' }
        } else {
            if ones >= zeros { b'0' } else { b'1' }
        };

        rows.retain(|row| row[i] == keep);
    }

    rows[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_solve_part_1() {
        let result = solve_part_1(EXAMPLE).unwrap();
        assert_eq!(result, 198);
    }

    #[test]
    fn test_oxygen_rating() {
        let input_vec = parse_to_vec(EXAMPLE).unwrap();
        // let map_vec = generate_hashmap(&input_vec);
        let result = rating(input_vec, true);
        assert_eq!(result, vec![b'1', b'0', b'1', b'1', b'1']);
    }

    #[test]
    fn test_co2_rating() {
        let input_vec = parse_to_vec(EXAMPLE).unwrap();
        let result = rating(input_vec, false);
        assert_eq!(result, vec![b'0', b'1', b'0', b'1', b'0'])
    }

    #[test]
    fn test_solve_part_2() {
        let result = solve_part_2(EXAMPLE).unwrap();
        assert_eq!(result, 230);
    }
}

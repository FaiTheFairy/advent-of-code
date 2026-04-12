use std::fs;

use anyhow::{Result, bail};

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let sol1 = solve_part_1(&input)?;
    println!("Part 1: {sol1}");

    let sol2 = solve_part_2(&input);
    println!("Part 2: {sol2}");

    Ok(())
}

fn solve_part_2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| encoded_len(line) - line.len())
        .sum()
}

fn solve_part_1(input: &str) -> Result<usize> {
    let mut total_code = 0;
    let mut total_memory = 0;

    for line in input.lines().filter(|l| !l.is_empty()) {
        total_code += line.len();
        total_memory += memory_len(line)?;
    }

    Ok(total_code - total_memory)
}

fn encoded_len(s: &str) -> usize {
    let mut len = 2;

    for byte in s.bytes() {
        match byte {
            b'"' | b'\\' => len += 2,
            _ => len += 1,
        }
    }

    len
}

fn memory_len(literal: &str) -> Result<usize> {
    let bytes = literal.as_bytes();

    let mut i = 1;
    let end = bytes.len() - 1;
    let mut len = 0;

    while i < end {
        match bytes[i] {
            b'\\' => match bytes[i + 1] {
                b'\\' | b'"' => {
                    len += 1;
                    i += 2;
                }
                b'x' => {
                    let h1 = bytes[i + 2];
                    let h2 = bytes[i + 3];

                    len += 1;
                    i += 4;
                }
                other => bail!(r#"unknown escape sequence \\{other} in literal: {literal}"#),
            },

            _ => {
                len += 1;
                i += 1;
            }
        }
    }

    Ok(len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoded_len_examples() {
        assert_eq!(encoded_len(r#""""#), 6);
        assert_eq!(encoded_len(r#""abc""#), 9);
        assert_eq!(encoded_len(r#""aaa\"aaa""#), 16);
        assert_eq!(encoded_len(r#""\x27""#), 11);
    }

    #[test]
    fn test_part_2_example() {
        let input: &str = concat!("\"\"\n", "\"abc\"\n", "\"aaa\\\"aaa\"\n", "\"\\x27\"\n",);

        assert_eq!(solve_part_2(input), 19);
    }

    #[test]
    fn test_memory_len_examples() {
        assert_eq!(memory_len(r#""""#).unwrap(), 0);
        assert_eq!(memory_len(r#""abc""#).unwrap(), 3);
        assert_eq!(memory_len(r#""aaa\"aaa""#).unwrap(), 7);
        assert_eq!(memory_len(r#""\x27""#).unwrap(), 1);
    }

    #[test]
    fn test_example_total() {
        let input: &str = concat!("\"\"\n", "\"abc\"\n", "\"aaa\\\"aaa\"\n", "\"\\x27\"\n",);

        assert_eq!(solve_part_1(input).unwrap(), 12);
    }
}

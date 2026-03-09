use anyhow::{Context, Result};
use std::{collections::HashMap, fs};

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let sol1 = solve_part_1(&input)?;
    println!("Part 1. power consumption = {sol1}");

    let sol2 = solve_part_2(&input)?;
    println!("Part 2. life support rating = {sol2}");
    Ok(())
}

fn solve_part_2(input: &str) -> Result<usize> {
    let input_vec = parse_to_vec(input);
    let oxygen = oxygen_generator_rating(&input_vec);
    let co2 = co2_scrubber_rating(&input_vec);
    let oxygen = usize::from_str_radix(
        &oxygen
            .iter()
            .map(|b| char::from_u32(*b as u32).context("Couldn't parse u8 to char"))
            .collect::<Result<String>>()?,
        2,
    )?;
    let co2 = usize::from_str_radix(
        &co2.iter()
            .map(|b| char::from_u32(*b as u32).context("Couldn't parse u8 to char"))
            .collect::<Result<String>>()?,
        2,
    )?;
    Ok(oxygen * co2)
}

fn oxygen_generator_rating(input_vec: &[Vec<u8>]) -> Vec<u8> {
    let mut out_vec = input_vec.to_vec();
    for i in 0..out_vec[0].len() {
        let map_vec = generate_hashmap(&out_vec);
        let max = if map_vec[i].get(&b'1') >= map_vec[i].get(&b'0') {
            b'1'
        } else {
            b'0'
        };
        out_vec.retain(|e| e[i] == max);
        if out_vec.len() == 1 {
            break;
        }
    }
    out_vec[0].clone()
}

fn co2_scrubber_rating(input_vec: &[Vec<u8>]) -> Vec<u8> {
    let mut out_vec = input_vec.to_vec();
    for i in 0..out_vec[0].len() {
        let map_vec = generate_hashmap(&out_vec);
        let min = if map_vec[i].get(&b'1') >= map_vec[i].get(&b'0') {
            b'0'
        } else {
            b'1'
        };
        out_vec.retain(|e| e[i] == min);
        if out_vec.len() == 1 {
            break;
        }
    }
    out_vec[0].clone()
}

fn solve_part_1(input: &str) -> Result<usize> {
    let input_vec = parse_to_vec(input);
    let map_vec = generate_hashmap(&input_vec);
    let gamma: String = get_gamma(&map_vec)
        .iter()
        .map(|b| char::from_u32(*b as u32).unwrap())
        .collect();
    let epsilon: String = get_epsilon(&map_vec)
        .iter()
        .map(|b| char::from_u32(*b as u32).unwrap())
        .collect();
    let gamma = usize::from_str_radix(&gamma, 2)?;
    let epsilon = usize::from_str_radix(&epsilon, 2)?;

    Ok(gamma * epsilon)
}

fn get_gamma(map_vec: &[HashMap<u8, usize>]) -> Vec<u8> {
    let mut out = Vec::new();
    for map in map_vec {
        out.push(*map.iter().max_by_key(|e| e.1).unwrap().0);
    }
    out
}

fn get_epsilon(map_vec: &[HashMap<u8, usize>]) -> Vec<u8> {
    let mut out = Vec::new();
    for map in map_vec {
        out.push(*map.iter().min_by_key(|e| e.1).unwrap().0);
    }
    out
}

fn generate_hashmap(input_vec: &[Vec<u8>]) -> Vec<HashMap<u8, usize>> {
    let len = input_vec.first().unwrap().len();
    let mut out = vec![HashMap::new(); len];
    for line in input_vec {
        for (idx, &c) in line.iter().enumerate() {
            out[idx].entry(c).and_modify(|e| *e += 1).or_insert(1);
        }
    }
    out
}

fn parse_to_vec(input: &str) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    for line in input.lines() {
        let line_vec: Vec<u8> = line.bytes().collect();
        out.push(line_vec);
    }

    out
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
    fn test_get_gamma() {
        let input_vec = parse_to_vec(EXAMPLE);
        let parsed = generate_hashmap(&input_vec);
        let result = get_gamma(&parsed);
        assert_eq!(result, vec![b'1', b'0', b'1', b'1', b'0']);
    }

    #[test]
    fn test_solve_part_1() {
        let result = solve_part_1(EXAMPLE).unwrap();
        assert_eq!(result, 198);
    }

    #[test]
    fn test_oxygen_rating() {
        let input_vec = parse_to_vec(EXAMPLE);
        // let map_vec = generate_hashmap(&input_vec);
        let result = oxygen_generator_rating(&input_vec);
        assert_eq!(result, vec![b'1', b'0', b'1', b'1', b'1']);
    }

    #[test]
    fn test_co2_rating() {
        let input_vec = parse_to_vec(EXAMPLE);
        let result = co2_scrubber_rating(&input_vec);
        assert_eq!(result, vec![b'0', b'1', b'0', b'1', b'0'])
    }

    #[test]
    fn test_solve_part_2() {
        let result = solve_part_2(EXAMPLE).unwrap();
        assert_eq!(result, 230);
    }
}

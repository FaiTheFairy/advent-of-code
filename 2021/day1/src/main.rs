use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let parsed = parse_to_vec(&input)?;
    let sol1 = solve_part_1(&parsed);
    println!("Part 1. Depth increases {sol1} times.");

    let sol2 = solve_part_2(&parsed);
    println!("Part 2. Depth increases {sol2} time when taking windows of size 3.");

    Ok(())
}

fn parse_to_vec(s: &str) -> Result<Vec<usize>> {
    let lines: Vec<usize> = s
        .lines()
        .map(str::parse::<usize>)
        .collect::<Result<_, _>>()?;
    Ok(lines)
}

fn solve_part_1(s: &[usize]) -> usize {
    s.windows(2).filter(|&w| w[0] < w[1]).count()
}

// For every overlapping window of size three,
// a, b, c, d -> [a, b, c], [b, c, d]
// we need to count the number of times this holds
// a + b + c < b + c + d
// which simplifies to
// a < d
fn solve_part_2(values: &[usize]) -> usize {
    values.array_windows().filter(|[a, _, _, d]| a < d).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "199
200
208
210
200
207
240
269
260
263";

    fn parsed_vec() -> Vec<usize> {
        parse_to_vec(EXAMPLE).unwrap()
    }

    #[test]
    fn test_solve_part_1() {
        let result = solve_part_1(&parsed_vec());
        assert_eq!(result, 7usize);
    }

    #[test]
    fn test_solve_part_2() {
        let result = solve_part_2(&parsed_vec());
        assert_eq!(result, 5usize);
    }
}

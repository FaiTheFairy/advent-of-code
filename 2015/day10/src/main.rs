use anyhow::Result;

fn main() -> Result<()> {
    let input = "1113122113";
    let sol1 = solve_sequence(input, 40);
    println!("Part 1: {sol1}");

    let sol2 = solve_sequence(input, 50);
    println!("Part 2: {sol2}");

    Ok(())
}

fn solve_sequence(input: &str, iterations: usize) -> usize {
    let mut seq: Vec<u8> = input.trim().bytes().collect();

    for _ in 0..iterations {
        seq = step(&seq);
    }

    seq.len()
}

fn step(input: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(2 * input.len());

    let mut i = 0;

    while i < input.len() {
        let digit = input[i];
        let mut count = 1;

        while i + count < input.len() && input[i + count] == digit {
            count += 1;
        }

        // append count
        for d in count.to_string().as_bytes() {
            out.push(*d);
        }

        // append original digit
        out.push(digit);

        i += count;
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_sequence() {
        let result = solve_sequence("1", 5);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_step() {
        assert_eq!(step(b"1"), b"11");
        assert_eq!(step(b"111221"), b"312211");
    }
}

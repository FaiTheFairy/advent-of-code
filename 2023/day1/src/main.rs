use std::fs;

fn main() {
    // part 1
    let contents = fs::read_to_string("./input.txt").unwrap();
    let sum: u32 = contents.lines().map(first_and_last_digits).sum();
    println!("Part 1. Sum is {sum}");

    // part 2
    let normalized = normalize(&contents);
    let sum2: u32 = normalized.lines().map(first_and_last_digits).sum();

    println!("Part 2. Sum is {sum2}");
}

/// Returns the first digit it encounters concatenated with the last digit
fn first_and_last_digits(input: &str) -> u32 {
    let first = input
        .chars()
        .find_map(|c| c.to_digit(10))
        .expect("No numerical digit found");
    let second = input
        .chars()
        .rev()
        .find_map(|c| c.to_digit(10))
        .expect("No numerical digit found");
    first * 10 + second
}

/// This replaces any instance of a spelled out digit with its corresponding letter.
/// The function handles overlaps by counting both numbers. e.g., twone results in `t2o1e`
fn normalize(input: &str) -> String {
    input
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

    const EXAMPLE2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

    #[test]
    fn test_first_and_last_digits() {
        let sum: u32 = EXAMPLE.lines().map(first_and_last_digits).sum();
        assert_eq!(sum, 142);
    }

    #[test]
    fn test_first_and_last_normalized() {
        let normalized = normalize(EXAMPLE2);
        let sum: u32 = normalized.lines().map(first_and_last_digits).sum();
        assert_eq!(sum, 281);
    }
}

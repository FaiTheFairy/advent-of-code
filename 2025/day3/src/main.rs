use std::io::{BufRead, BufReader};

// The batteries are arranged into banks; each line of digits in your input corresponds to a single bank of batteries.
// Within each bank, you need to turn on exactly two batteries; the joltage that the bank produces is equal to the
// number formed by the digits on the batteries you've turned on. For example, if you have a bank like 12345 and you
// turn on batteries 2 and 4, the bank would produce 24 jolts. (You cannot rearrange batteries.)
// You'll need to find the largest possible joltage each bank can produce.
fn largest_two_digit_number(line: &str) -> u8 {
    let digits: Vec<u8> = line
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    let mut best = 0u8;

    for i in 0..digits.len() {
        for j in i + 1..digits.len() {
            let value = digits[i] * 10 + digits[j];
            if value > best {
                best = value;
            }
        }
    }
    best
}

fn largest_k_digit_subsequence(line: &str, k: usize) -> Option<u128> {
    let digits: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
    if k == 0 {
        return Some(0);
    }
    if digits.len() < k {
        return None;
    }

    // best[len] = best value formable using exactly `len` digits
    let mut best: Vec<Option<u128>> = vec![None; k + 1];
    best[0] = Some(0);

    for d in digits {
        let digit = d as u128;

        // update from high to low so each input digit is used at most once
        for len in (1..=k).rev() {
            let Some(prev) = best[len - 1] else {
                continue;
            };

            let candidate = prev * 10 + digit;

            best[len] = match best[len] {
                Some(current) => Some(current.max(candidate)),
                None => Some(candidate),
            };
        }
    }

    best[k]
}

fn largest_twelve_digit_subsequence(line: &str) -> Option<u128> {
    largest_k_digit_subsequence(line, 12)
}

fn main() {
    let mut joltage: Vec<u128> = vec![];
    let file = std::fs::File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        joltage.push(largest_twelve_digit_subsequence(&line.unwrap()).unwrap());
    }
    let mut sum = 0u128;
    for value in joltage {
        sum += value as u128;
    }
    println!("{}", sum);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_two_digits() {
        let input = "987654321111111";
        let result = largest_two_digit_number(input);
        let expected = 98u8;
        assert_eq!(result, expected);
    }

    #[test]
    fn first_and_last() {
        let input = "811111111111119";
        let result = largest_two_digit_number(input);
        let expected = 89u8;
        assert_eq!(result, expected);
    }

    #[test]
    fn last_two_digits() {
        let input = "234234234234278";
        let result = largest_two_digit_number(input);
        let expected = 78u8;
        assert_eq!(result, expected);
    }

    #[test]
    fn random() {
        let input = "818181911112111";
        let result = largest_two_digit_number(input);
        let expected = 92u8;
        assert_eq!(result, expected);
    }

    #[test]
    fn twelve_digit() {
        let input = "987654321111111";
        let result = largest_twelve_digit_subsequence(input).unwrap();
        let expected = 987654321111u128;
        assert_eq!(result, expected);
    }
}

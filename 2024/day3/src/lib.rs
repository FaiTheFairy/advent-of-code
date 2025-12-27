pub fn solve_part1(input: &str) -> u64 {
    let bytes = input.as_bytes();
    let mut i = 0;
    let mut sum: u64 = 0;

    while i + 4 < bytes.len() {
        // look for "mul("
        if bytes[i] == b'm' && bytes[i + 1] == b'u' && bytes[i + 2] == b'l' && bytes[i + 3] == b'('
        {
            let mut j = i + 4;

            // parse first number (1-3 digits)
            let mut x: u64 = 0;
            let mut digits = 0;
            while j < bytes.len() && bytes[j].is_ascii_digit() {
                x = x * 10 + (bytes[j] - b'0') as u64;
                digits += 1;
                j += 1;
            }
            if digits == 0 || digits > 3 || j >= bytes.len() || bytes[j] != b',' {
                i += 1;
                continue;
            }
            j += 1;

            // parse second number (1-3 digits)
            let mut y: u64 = 0;
            digits = 0;
            while j < bytes.len() && bytes[j].is_ascii_digit() {
                y = y * 10 + (bytes[j] - b'0') as u64;
                digits += 1;
                j += 1;
            }
            if digits == 0 || digits > 3 || j >= bytes.len() || bytes[j] != b')' {
                i += 1;
                continue;
            }

            // valid instruction
            sum += x * y;
            i = j + 1;
        } else {
            i += 1;
        }
    }

    sum
}

pub fn solve_part2(input: &str) -> u64 {
    let bytes = input.as_bytes();
    let mut i = 0;
    let mut sum: u64 = 0;
    let mut enabled = true;

    while i < bytes.len() {
        // do()
        if i + 3 < bytes.len()
            && bytes[i] == b'd'
            && bytes[i + 1] == b'o'
            && bytes[i + 2] == b'('
            && bytes[i + 3] == b')'
        {
            enabled = true;
            i += 4;
            continue;
        }

        // don't
        if i + 6 < bytes.len()
            && bytes[i] == b'd'
            && bytes[i + 1] == b'o'
            && bytes[i + 2] == b'n'
            && bytes[i + 3] == b'\''
            && bytes[i + 4] == b't'
            && bytes[i + 5] == b'('
            && bytes[i + 6] == b')'
        {
            enabled = false;
            i += 7;
            continue;
        }

        // look for "mul("
        if i + 3 < bytes.len()
            && bytes[i] == b'm'
            && bytes[i + 1] == b'u'
            && bytes[i + 2] == b'l'
            && bytes[i + 3] == b'('
        {
            let mut j = i + 4;

            // parse first number (1-3 digits)
            let mut x: u64 = 0;
            let mut digits = 0;
            while j < bytes.len() && bytes[j].is_ascii_digit() {
                x = x * 10 + (bytes[j] - b'0') as u64;
                digits += 1;
                j += 1;
            }
            if digits == 0 || digits > 3 || j >= bytes.len() || bytes[j] != b',' {
                i += 1;
                continue;
            }
            j += 1;

            // parse second number (1-3 digits)
            let mut y: u64 = 0;
            digits = 0;
            while j < bytes.len() && bytes[j].is_ascii_digit() {
                y = y * 10 + (bytes[j] - b'0') as u64;
                digits += 1;
                j += 1;
            }
            if digits == 0 || digits > 3 || j >= bytes.len() || bytes[j] != b')' {
                i += 1;
                continue;
            }

            // valid instruction
            if enabled {
                sum += x * y;
            }
            i = j + 1;
            continue;
        }
        i += 1;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_official_example() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(solve_part1(input), 161);
    }

    #[test]
    fn ignores_invalid_variants() {
        // All of these should be ignored:
        // - mul(4*        (bad char)
        // - mul(6,9!      (missing ')')
        // - ?(12,34)      (wrong prefix)
        // - mul ( 2 , 4 ) (spaces not allowed by the grammar)
        let input = "mul(4* mul(6,9! ?(12,34) mul ( 2 , 4 )";
        assert_eq!(solve_part1(input), 0);
    }

    #[test]
    fn accepts_multiple_valid_in_a_row() {
        let input = "mul(1,2)mul(3,4)mul(5,6)";
        assert_eq!(solve_part1(input), 1 * 2 + 3 * 4 + 5 * 6);
    }

    #[test]
    fn accepts_1_to_3_digit_numbers_only() {
        // valid: 0..999 as long as 1-3 digits (including 000 isn't possible here because "000" parses to 0, but it's still 3 digits)
        let input = "mul(9,9)mul(99,1)mul(100,2)mul(999,3)";
        assert_eq!(solve_part1(input), 81 + 99 + 200 + 2997);

        // invalid: 4 digits should be ignored
        let input2 = "mul(1000,2)mul(2,1000)mul(1234,5)";
        assert_eq!(solve_part1(input2), 0);
    }

    #[test]
    fn requires_exact_punctuation() {
        // wrong brackets, missing comma, extra comma, missing paren
        let input = "mul[2,3] mul(2 3) mul(2,,3) mul(2,3";
        assert_eq!(solve_part1(input), 0);
    }

    #[test]
    fn does_not_get_confused_by_prefixes() {
        // "xmul(2,4)" still contains "mul(2,4)" starting at index 1, so it should count.
        let input = "xmul(2,4)";
        assert_eq!(solve_part1(input), 8);

        // "do_not_mul(5,5)" also contains "mul(5,5)" as a substring.
        let input2 = "do_not_mul(5,5)";
        assert_eq!(solve_part1(input2), 25);
    }

    #[test]
    fn handles_trailing_and_leading_garbage() {
        let input = "!!!mul(7,8)???";
        assert_eq!(solve_part1(input), 56);
    }

    #[test]
    fn empty_and_no_matches() {
        assert_eq!(solve_part1(""), 0);
        assert_eq!(solve_part1("abcdefg"), 0);
    }

    #[test]
    fn test_part2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(solve_part2(input), 48);
    }
}

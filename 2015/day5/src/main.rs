use std::fs;

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let sol1 = input.lines().filter(|s| is_nice_v1(s)).count();
    println!("Part 1: {sol1}");

    let sol2 = input.lines().filter(|s| is_nice_v2(s)).count();
    println!("Part 2: {sol2}");

    Ok(())
}

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn is_nice_v1(string: &str) -> bool {
    let vowel_count = string.chars().filter(|c| VOWELS.contains(c)).count();
    if vowel_count < 3 {
        return false;
    }

    let chars = string.as_bytes();

    let has_forbidden_pair = chars
        .array_windows::<2>()
        .any(|w| matches!(w, [b'a', b'b'] | [b'c', b'd'] | [b'p', b'q'] | [b'x', b'y']));

    if has_forbidden_pair {
        return false;
    }

    chars.array_windows::<2>().any(|[a, b]| a == b)
}

fn is_nice_v2(string: &str) -> bool {
    let chars = string.as_bytes();

    let cond1 = has_repeated_non_overlapping_pair(chars);
    let cond2 = chars.array_windows::<3>().any(|[a, _, c]| a == c);

    cond1 && cond2
}

fn has_repeated_non_overlapping_pair(ascii_bytes: &[u8]) -> bool {
    for i in 0..ascii_bytes.len().saturating_sub(1) {
        let pair = (ascii_bytes[i], ascii_bytes[i + 1]);

        for j in i + 2..ascii_bytes.len().saturating_sub(1) {
            if (ascii_bytes[j], ascii_bytes[j + 1]) == pair {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_nice_v2() {
        assert!(is_nice_v2("qjhvhtzxzqqjkmpb"));
        assert!(is_nice_v2("xxyxx"));

        assert!(
            !is_nice_v2("uurcxstgmygtbstg"),
            "no repeated letter with a single letter in between"
        );
        assert!(
            !is_nice_v2("ieodomkazucvgmuy"),
            "no pairs that appear twice"
        );
    }

    #[test]
    fn test_is_nice_v1() {
        assert!(is_nice_v1("ugknbfddgicrmopn"));
        assert!(is_nice_v1("aaa"));

        assert!(!is_nice_v1("jchzalrnumimnmhp"), "no double letter");
        assert!(!is_nice_v1("haegwjzuvuyypxyu"), "contains \"xy\"");
        assert!(!is_nice_v1("dvszwmarrgswjxmb"), "contains only one vowel");
    }
}

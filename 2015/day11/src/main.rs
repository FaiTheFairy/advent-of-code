use anyhow::{Context, Result};

fn main() -> Result<()> {
    let input = "hepxcrrq";

    let sol1 = next_password(input).context("no next password found")?;
    println!("Part 1: {sol1}");

    let sol2 = next_password(&sol1).context("no second password found")?;
    println!("Part 2: {sol2}");

    Ok(())
}

fn next_password(start: &str) -> Option<String> {
    let mut password = start.as_bytes().to_vec();

    loop {
        increment_password(&mut password);

        if is_valid_password(&password) {
            return String::from_utf8(password).ok();
        }
    }
}

fn increment_password(password: &mut [u8]) {
    let mut i = password.len();

    while i > 0 {
        i -= 1;
        password[i] += 1;

        if password[i] > b'z' {
            password[i] = b'a';
            continue;
        }

        break;
    }
}

fn is_valid_password(password: &[u8]) -> bool {
    has_straight(password) && has_no_forbidden_letters(password) && has_two_pairs(password)
}

fn has_straight(password: &[u8]) -> bool {
    password
        .windows(3)
        .any(|w| w[0] + 1 == w[1] && w[1] + 1 == w[2])
}

fn has_no_forbidden_letters(password: &[u8]) -> bool {
    !password.iter().any(|&b| matches!(b, b'i' | b'o' | b'l'))
}

fn has_two_pairs(password: &[u8]) -> bool {
    let mut pair_count = 0;
    let mut i = 0;

    while i + 1 < password.len() {
        if password[i] == password[i + 1] {
            pair_count += 1;
            i += 2;

            if pair_count >= 2 {
                return true;
            }
        } else {
            i += 1;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples_validity() {
        assert!(!is_valid_password(b"hijklmmn"));
        assert!(!is_valid_password(b"abbceffg"));
        assert!(!is_valid_password(b"abbcegjk"));
    }

    #[test]
    fn test_next_password_examples() {
        assert_eq!(next_password("abcdefgh").unwrap(), "abcdffaa");
        assert_eq!(next_password("ghijklmn").unwrap(), "ghjaabcc");
    }
}

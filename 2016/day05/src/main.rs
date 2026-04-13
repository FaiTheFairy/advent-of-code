fn main() {
    let door_id = "uqwqemis";
    let password = password_part_1(door_id);
    println!("Part 1: {password}");

    let password2 = password_part_2(door_id);
    println!("Part 2: {password2}");
}

fn password_part_1(door_id: &str) -> String {
    let mut password = String::with_capacity(8);
    let mut index = 0;

    while password.len() < 8 {
        let input = format!("{door_id}{index}");
        let digest = md5::compute(input).0;

        if is_interesting(&digest) {
            let sixth_hex_digit = nibble_to_hex(digest[2] & 0x0f);
            password.push(sixth_hex_digit);
        }

        index += 1;
    }

    password
}

fn password_part_2(door_id: &str) -> String {
    let mut password = [None; 8];
    let mut filled = 0;
    let mut index = 0;

    while filled < 8 {
        let input = format!("{door_id}{index}");
        let digest = md5::compute(input).0;

        if is_interesting(&digest) {
            let position_nibble = digest[2] & 0x0f;
            if position_nibble < 8 {
                let position = position_nibble as usize;

                if password[position].is_none() {
                    let ch = nibble_to_hex(digest[3] >> 4);
                    password[position] = Some(ch);
                    filled += 1;
                }
            }
        }
        index += 1;
    }

    password.into_iter().map(|c| c.unwrap()).collect()
}

/// Returns true if the MD5 digest starts with five leading hex zeroes.
///
/// Each hex digit represents 4 bits, so "00000" means the first 20 bits are zero.
/// The digest is 16 bytes (128 bits), so we check:
///
/// - digest[0] == 0 -> first 8 bits (2 hex digits) are 0
/// - digest[1] == 0 -> next 8 bits (2 more hex digits) are 0
/// - digest[2] >> 4 == 0 -> high 4 bits (5th hex digit) are 0
fn is_interesting(digest: &[u8; 16]) -> bool {
    digest[0] == 0 && digest[1] == 0 && (digest[2] >> 4) == 0
}

/// Converts a 4-bit value (0–15) into its corresponding hexadecimal character.
///
/// A "nibble" is half a byte:
/// - 0..=9  -> '0'..='9'
/// - 10..=15 -> 'a'..='f'
///
/// We extract the sixth hex digit of the hash, which is the
/// *low nibble* of digest[2] (i.e., digest[2] & 0x0f), and convert it to a char.
///
/// Assumes input is in 0..=15.
fn nibble_to_hex(nibble: u8) -> char {
    match nibble {
        0..=9 => char::from(b'0' + nibble),
        10..=15 => char::from(b'a' + (nibble - 10)),
        _ => unreachable!("nibble must be in 0..=15"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_2() {
        assert_eq!(password_part_2("abc"), "05ace8e3");
    }

    #[test]
    fn test_example_part_1() {
        assert_eq!(password_part_1("abc"), "18f47a30");
    }

    #[test]
    fn test_nibble_to_hex() {
        assert_eq!(nibble_to_hex(0), '0');
        assert_eq!(nibble_to_hex(9), '9');
        assert_eq!(nibble_to_hex(10), 'a');
        assert_eq!(nibble_to_hex(15), 'f');
    }
}

use std::time::SystemTime;

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let key = b"iwrupvqb";

    let now = SystemTime::now();
    let sol1 = find_number(key, 5).context("no solution found for part 1")?;
    let elapsed = now.elapsed()?;
    println!("Part 1: {sol1}");
    println!("elapsed: {elapsed:?}");

    let now = SystemTime::now();
    let sol2 = find_number(key, 6).context("no solution found for part 2")?;
    let elapsed = now.elapsed()?;
    println!("Part 2: {sol2}");
    println!("elapsed: {elapsed:?}");

    Ok(())
}

fn find_number(key: &[u8], zeros: usize) -> Option<usize> {
    let mut buffer: Vec<u8> = Vec::with_capacity(key.len() + 20);
    buffer.extend_from_slice(key);

    for n in 0..1_000_000_000 {
        buffer.truncate(key.len());
        append_decimal(&mut buffer, n);

        let digest = md5::compute(&buffer).0;

        let ok = match zeros {
            5 => digest[0] == 0 && digest[1] == 0 && digest[2] < 0x10,
            6 => digest[0] == 0 && digest[1] == 0 && digest[2] == 0,
            7 => digest[0] == 0 && digest[1] == 0 && digest[2] == 0 && digest[3] < 0x10,
            _ => unreachable!("unsuppoted zero count"),
        };

        if ok {
            return Some(n);
        }
    }

    None
}

/// Append deicmal representation of `n` as ASCII bytes.
/// We push digits via n % 10 (least significant first), so they are reversed,
/// then reverse the appended range to restore correct order.
/// This avoids allocation compared to `to_string`.
fn append_decimal(buf: &mut Vec<u8>, mut n: usize) {
    if n == 0 {
        buf.push(b'0');
        return;
    }

    let start = buf.len();

    while n > 0 {
        buf.push(b'0' + (n % 10) as u8);
        n /= 10;
    }

    buf[start..].reverse();
}

use std::fs;

use anyhow::{Context, Result, bail};

fn main() -> Result<()> {
    let input = fs::read_to_string("./input.txt").context("failed to read ./input.txt")?;
    let s = input.trim();

    let part1 = first_marker_end(s, 4).context("part 1: no start-of-packet marker found")?;
    println!("Part 1. characters processed before marker = {part1}");

    let part2 = first_marker_end(s, 14).context("part 2: no start-of-message marker found")?;
    println!("Part 2. characters processed before marker = {part2}");

    Ok(())
}

/// Returns the number of characters processed up to (and including) the end of the
/// first window of length `window` where all bytes are distinct.
fn first_marker_end(s: &str, window: usize) -> Result<usize> {
    let bytes = s.as_bytes();
    if window == 0 {
        bail!("window size cannot be 0");
    }
    if bytes.len() < window {
        bail!(
            "input length {} is shorter than window size {}",
            bytes.len(),
            window
        );
    }

    // Sliding window with per-byte counts.
    let mut counts = [0u8; 256];
    let mut dupes: usize = 0;

    // Seed the first window [0..window)
    for &b in &bytes[..window] {
        let idx = b as usize;
        let prev = counts[idx];
        counts[idx] = prev + 1;
        if prev + 1 == 2 {
            dupes += 1;
        }
    }

    if dupes == 0 {
        return Ok(window);
    }

    // Slide: window ends at `end` (exclusive), starts at `end - window`
    for end in window + 1..=bytes.len() {
        let out_b = bytes[end - window - 1] as usize;
        let out_prev = counts[out_b];
        counts[out_b] = out_prev - 1;
        if out_prev == 2 {
            dupes -= 1; // went 2 -> 1, so one fewer duplicated byte
        }

        let in_b = bytes[end - 1] as usize;
        let in_prev = counts[in_b];
        counts[in_b] = in_prev + 1;
        if in_prev + 1 == 2 {
            dupes += 1; // went 1 -> 2, so one more duplicated byte
        }

        if dupes == 0 {
            return Ok(end);
        }
    }

    bail!("no all-distinct window of length {window} found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part_1() -> Result<()> {
        assert_eq!(first_marker_end("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4)?, 7);
        assert_eq!(first_marker_end("bvwbjplbgvbhsrlpgdmjqwftvncz", 4)?, 5);
        assert_eq!(first_marker_end("nppdvjthqldpwncqszvftbrmjlhg", 4)?, 6);
        assert_eq!(
            first_marker_end("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4)?,
            10
        );
        assert_eq!(first_marker_end("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4)?, 11);
        Ok(())
    }

    #[test]
    fn examples_part_2() -> Result<()> {
        assert_eq!(first_marker_end("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14)?, 19);
        assert_eq!(first_marker_end("bvwbjplbgvbhsrlpgdmjqwftvncz", 14)?, 23);
        assert_eq!(first_marker_end("nppdvjthqldpwncqszvftbrmjlhg", 14)?, 23);
        assert_eq!(
            first_marker_end("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14)?,
            29
        );
        assert_eq!(
            first_marker_end("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14)?,
            26
        );
        Ok(())
    }

    #[test]
    fn window_too_large() {
        let err = first_marker_end("abc", 4).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("shorter than window size"));
    }
}

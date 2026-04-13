use std::{fs, str::FromStr};

use anyhow::{Context, Result, ensure};

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let sol1 = decompressed_len::<V1>(&input)?;
    println!("Part 1: {sol1}");

    let sol2 = decompressed_len::<V2>(&input)?;
    println!("Part 2: {sol2}");

    Ok(())
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Marker {
    len: usize,
    times: usize,
}

impl FromStr for Marker {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (len, times) = s.split_once('x').context("marker missing 'x'")?;
        let len = len.parse().context("invalid marker length")?;
        let times = times.parse().context("invalid marker repeat count")?;
        Ok(Self { len, times })
    }
}

trait Expansion {
    fn repeated_len(data: &str, marker: Marker) -> Result<usize>;
}

struct V1;
struct V2;

impl Expansion for V1 {
    fn repeated_len(_data: &str, marker: Marker) -> Result<usize> {
        Ok(marker.len * marker.times)
    }
}

impl Expansion for V2 {
    fn repeated_len(data: &str, marker: Marker) -> Result<usize> {
        Ok(decompressed_len::<V2>(data)? * marker.times)
    }
}

fn decompressed_len<E: Expansion>(input: &str) -> Result<usize> {
    let filtered: String = input.chars().filter(|c| !c.is_whitespace()).collect();
    let bytes = filtered.as_bytes();

    let mut i = 0;
    let mut total = 0;

    while i < bytes.len() {
        if bytes[i] != b'(' {
            total += 1;
            i += 1;
            continue;
        }

        let close = bytes[i + 1..]
            .iter()
            .position(|&b| b == b')')
            .map(|offset| i + 1 + offset)
            .context("missing closing ')'")?;

        let marker_str =
            std::str::from_utf8(&bytes[i + 1..close]).context("marker not valid utf-8")?;
        let marker: Marker = marker_str.parse()?;

        let data_start = close + 1;
        let data_end = data_start + marker.len;
        ensure!(data_end <= bytes.len(), "marker extends past end of input");

        let data =
            std::str::from_utf8(&bytes[data_start..data_end]).context("data not valid utf-8")?;

        total += E::repeated_len(data, marker)?;
        i = data_end;
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decompressed_len_v2() {
        assert_eq!(decompressed_len::<V2>("(3x3)XYZ").unwrap(), 9);
        assert_eq!(
            decompressed_len::<V2>("X(8x2)(3x3)ABCY").unwrap(),
            6 * 3 + 2
        );
        assert_eq!(
            decompressed_len::<V2>("(27x12)(20x12)(13x14)(7x10)(1x12)A").unwrap(),
            241_920
        );
        assert_eq!(
            decompressed_len::<V2>("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN")
                .unwrap(),
            445
        );
    }

    #[test]
    fn test_decompressed_len_v1() {
        assert_eq!(decompressed_len::<V1>("ADVENT").unwrap(), 6);
        assert_eq!(decompressed_len::<V1>("A(1x5)BC").unwrap(), 7);
        assert_eq!(decompressed_len::<V1>("(3x3)XYZ").unwrap(), 9);
        assert_eq!(decompressed_len::<V1>("A(2x2)BCD(2x2)EFG").unwrap(), 11);
        assert_eq!(decompressed_len::<V1>("(6x1)(1x3)A").unwrap(), 6);
    }
}

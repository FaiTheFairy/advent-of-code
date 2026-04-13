#![allow(dead_code)]

use anyhow::{Context, Result, bail};
use std::{collections::HashMap, fs, str::FromStr};

fn main() -> Result<()> {
    let input: Input = fs::read_to_string("input.txt")?.parse()?;

    let sol1 = input.sum_of_real_sector_ids();
    println!("Part 1: {sol1}");

    let sol2 = input
        .iter()
        .filter(|r| r.is_real())
        .find(|r| r.decrypted_name().contains("north"))
        .context("no names contain 'north'")?
        .id;
    println!("Part 2: {sol2}");

    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Input(Vec<Room>);

impl Input {
    fn sum_of_real_sector_ids(&self) -> usize {
        self.iter()
            .filter(|room| room.is_real())
            .map(|room| room.id)
            .sum()
    }

    fn iter(&self) -> impl Iterator<Item = &Room> {
        self.0.iter()
    }
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let inner = s
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        Ok(Self(inner))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Room {
    encrypted_name: String,
    id: usize,
    checksum: [char; 5],
}

impl Room {
    fn decrypted_name(&self) -> String {
        let shift = (self.id % 26) as u8;

        self.encrypted_name
            .chars()
            .map(|ch| match ch {
                'a'..='z' => {
                    let offset = ch as u8 - b'a';
                    let rotated = (offset + shift) % 26;
                    (b'a' + rotated) as char
                }
                '-' => ' ',
                _ => ch,
            })
            .collect()
    }

    fn is_real(&self) -> bool {
        self.computed_checksum() == self.checksum
    }

    fn computed_checksum(&self) -> [char; 5] {
        let mut counts: HashMap<char, usize> = HashMap::new();

        for ch in self.encrypted_name.chars().filter(|&c| c != '-') {
            *counts.entry(ch).or_insert(0) += 1;
        }

        let mut letters: Vec<(char, usize)> = counts.into_iter().collect();

        letters.sort_unstable_by(|(left_char, left_count), (right_char, right_count)| {
            right_count
                .cmp(left_count)
                .then_with(|| left_char.cmp(right_char))
        });

        [
            letters[0].0,
            letters[1].0,
            letters[2].0,
            letters[3].0,
            letters[4].0,
        ]
    }
}

impl FromStr for Room {
    type Err = anyhow::Error;

    /// parses `aa-b-x-y-z-123[abxyz]` to
    /// ```
    /// Room {
    ///     encrypted_name: vec!['a', 'a', '-', 'b', '-', 'x', '-', 'y', '-', 'z'],
    ///     id: 123,
    ///     checksum: ['a', 'b', 'x', 'y', 'z' ]
    /// }
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, checksum_part) = s.split_once('[').context("missing '[' in room")?;

        let checksum_part = checksum_part
            .strip_suffix(']')
            .context("missing closing ']' in room")?;

        let (name_part, id_part) = left
            .rsplit_once('-')
            .context("missing '-' before sector id")?;

        let encrypted_name: String = name_part.to_string();
        let id = id_part.parse().context("invalid sector id")?;
        let checksum = parse_checksum(checksum_part)?;

        Ok(Self {
            encrypted_name,
            id,
            checksum,
        })
    }
}

fn parse_checksum(s: &str) -> Result<[char; 5]> {
    let mut chars = s.chars();

    let checksum = [
        chars.next().context("checksum too short")?,
        chars.next().context("checksum too short")?,
        chars.next().context("checksum too short")?,
        chars.next().context("checksum too short")?,
        chars.next().context("checksum too short")?,
    ];

    if chars.next().is_some() {
        bail!("checksum too long");
    }

    Ok(checksum)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_decrypt_name() {
        let room = Room {
            encrypted_name: "qzmt-zixmtkozy-ivhz".to_string(),
            id: 343,
            checksum: ['a', 'b', 'c', 'd', 'e'],
        };

        assert_eq!(room.decrypted_name(), "very encrypted name");
    }

    #[test]
    fn test_real_room() {
        let room: Room = "aaaaa-bbb-z-y-x-123[abxyz]".parse().unwrap();
        assert!(room.is_real());
    }

    #[test]
    fn test_real_room_tie_breaking() {
        let room: Room = "a-b-c-d-e-f-g-h-987[abcde]".parse().unwrap();
        assert!(room.is_real());
    }

    #[test]
    fn test_not_real_room() {
        let room: Room = "totally-real-room-200[decoy]".parse().unwrap();
        assert!(!room.is_real());
    }

    #[test]
    fn test_parse_room_basic() {
        let room: Room = "aaaaa-bbb-z-y-x-123[abxyz]".parse().unwrap();

        assert_eq!(room.encrypted_name, "aaaaa-bbb-z-y-x");
        assert_eq!(room.id, 123);
        assert_eq!(room.checksum, ['a', 'b', 'x', 'y', 'z']);
    }
}

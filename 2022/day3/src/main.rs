use std::{
    collections::{HashSet, hash_set::Union},
    fs,
};

use anyhow::{Context, Error, Result, ensure};

fn main() -> Result<()> {
    let input = fs::read_to_string("./input.txt")?;
    let rucksacks = Rucksacks::try_from(input.as_str())?;
    let sum = rucksacks.sum_priorities_of_repeated();
    println!("Part 1. sum of priorities of repeated items = {sum}");

    let sum2 = rucksacks.sum_of_priorities_of_badges();
    println!("Part 2. sum of priorities of badges = {sum2}");
    Ok(())
}

struct Rucksacks(Vec<Rucksack>);

impl Rucksacks {
    fn sum_priorities_of_repeated(&self) -> usize {
        self.0
            .iter()
            .map(Rucksack::repeated_item)
            .map(|c: CasedAlphabet| CasedAlphabet::priority(&c) as usize)
            .sum()
    }

    fn sum_of_priorities_of_badges(&self) -> usize {
        self.0
            .chunks_exact(3)
            .map(|chunk| {
                let a = chunk[0].combined();
                let b = chunk[1].combined();
                let c = chunk[2].combined();

                let badge = a
                    .intersection(&b)
                    .find(|&item| c.contains(item))
                    .expect("Can't find similar type in chunk");
                badge.priority() as usize
            })
            .sum()
    }
}

impl TryFrom<&str> for Rucksacks {
    type Error = Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let rucksacks: Vec<Rucksack> = value
            .lines()
            .map(Rucksack::try_from)
            .collect::<Result<Vec<_>>>()?;
        Ok(Rucksacks(rucksacks))
    }
}

#[derive(Debug)]
struct Rucksack {
    compartment1: HashSet<CasedAlphabet>,
    compartment2: HashSet<CasedAlphabet>,
}

impl TryFrom<&str> for Rucksack {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim();
        ensure!(
            value.len().is_multiple_of(2),
            "Rutsack doesn't have an even number of items"
        );

        let mid = value.len() / 2;

        let (s1, s2) = value.split_at(mid);

        let compartment1: HashSet<CasedAlphabet> =
            s1.bytes().map(TryInto::try_into).collect::<Result<_>>()?;
        let compartment2: HashSet<CasedAlphabet> =
            s2.bytes().map(TryInto::try_into).collect::<Result<_>>()?;
        Ok(Self {
            compartment1,
            compartment2,
        })
    }
}

impl Rucksack {
    fn repeated_item(&self) -> CasedAlphabet {
        self.compartment1
            .intersection(&self.compartment2)
            .next()
            .copied()
            .unwrap()
    }

    fn combined(&self) -> HashSet<CasedAlphabet> {
        self.compartment1
            .union(&self.compartment2)
            .copied()
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CasedAlphabet(u8);

impl TryFrom<u8> for CasedAlphabet {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        ensure!(value.is_ascii_alphabetic());
        Ok(Self(value))
    }
}

impl CasedAlphabet {
    fn priority(&self) -> u8 {
        let b = self.0;
        if b.is_ascii_lowercase() {
            b - b'a' + 1
        } else {
            b - b'A' + 27
        }
    }
}

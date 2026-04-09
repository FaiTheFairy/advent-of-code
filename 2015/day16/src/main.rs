use std::{collections::HashMap, fs, str::FromStr};

use anyhow::{Context, Result, bail};

fn main() -> Result<()> {
    let list: List = fs::read_to_string("input.txt")?.parse()?;

    let desired_sue = Sue {
        id: 0,
        properties: HashMap::from_iter([
            (Property::Children, 3),
            (Property::Cats, 7),
            (Property::Samoyeds, 2),
            (Property::Pomeranians, 3),
            (Property::Akitas, 0),
            (Property::Vizslas, 0),
            (Property::Goldfish, 5),
            (Property::Trees, 3),
            (Property::Cars, 2),
            (Property::Perfumes, 1),
        ]),
    };

    let sol1 = list
        .find_desired_sue_v1(&desired_sue)
        .context("sue not found")?;
    println!("Part 1: {}", sol1.id);

    let sol2 = list
        .find_desired_sue_v2(&desired_sue)
        .context("sue not found")?;
    println!("Part 2: {}", sol2.id);

    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct List {
    aunts: Vec<Sue>,
}

impl List {
    fn find_desired_sue_v1(&self, candidate: &Sue) -> Option<&Sue> {
        self.aunts
            .iter()
            .find(|s| s.matches_candidate_part_1(candidate))
    }

    fn find_desired_sue_v2(&self, candidate: &Sue) -> Option<&Sue> {
        self.aunts
            .iter()
            .find(|s| s.matches_candidate_part_2(candidate))
    }
}

impl FromStr for List {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let aunts = s
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        Ok(Self { aunts })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Sue {
    id: usize,
    properties: HashMap<Property, usize>,
}

impl Sue {
    fn matches_candidate_part_1(&self, candidate: &Sue) -> bool {
        candidate
            .properties
            .iter()
            .all(|(property, candidate_value)| {
                self.properties
                    .get(property)
                    .is_none_or(|v| *v == *candidate_value)
            })
    }

    fn matches_candidate_part_2(&self, candidate: &Sue) -> bool {
        candidate
            .properties
            .iter()
            .all(|(property, candidate_value)| {
                self.properties
                    .get(property)
                    .is_none_or(|v| match property {
                        Property::Children
                        | Property::Cars
                        | Property::Vizslas
                        | Property::Akitas
                        | Property::Perfumes
                        | Property::Samoyeds => v == candidate_value,
                        Property::Trees | Property::Cats => v > candidate_value,
                        Property::Goldfish | Property::Pomeranians => v < candidate_value,
                    })
            })
    }
}

impl FromStr for Sue {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (sue, properties) = s.split_once(':').context("entry missing colon")?;
        let id = sue
            .strip_prefix("Sue ")
            .context("entry missing 'Sue'")?
            .parse()?;

        let properties = properties
            .split(',')
            .map(|part| -> Result<(Property, usize)> {
                let (name, value) = part.trim().split_once(':').context("bad proprty")?;

                let property = name.parse()?;
                let value = value.trim().parse()?;

                Ok((property, value))
            })
            .collect::<Result<_, _>>()?;

        Ok(Self { id, properties })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Property {
    Children,
    Cars,
    Vizslas,
    Akitas,
    Perfumes,
    Goldfish,
    Trees,
    Cats,
    Pomeranians,
    Samoyeds,
}

impl FromStr for Property {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "children" => Ok(Self::Children),
            "cars" => Ok(Self::Cars),
            "vizslas" => Ok(Self::Vizslas),
            "akitas" => Ok(Self::Akitas),
            "perfumes" => Ok(Self::Perfumes),
            "goldfish" => Ok(Self::Goldfish),
            "trees" => Ok(Self::Trees),
            "cats" => Ok(Self::Cats),
            "pomeranians" => Ok(Self::Pomeranians),
            "samoyeds" => Ok(Self::Samoyeds),
            _ => bail!("unknown property: {s}"),
        }
    }
}

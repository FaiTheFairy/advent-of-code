use std::{fs, str::FromStr};

use anyhow::{Result, anyhow, bail, ensure};

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let sol1 = solve_part_1(&input)?;
    println!("Part 1. number of valid passports in batch file = {sol1}");

    let sol2 = solve_part_2(&input)?;
    println!("Part 2. number of valid passports in batch file = {sol2}");

    Ok(())
}

fn solve_part_1(input: &str) -> Result<usize> {
    let batch = input.parse::<PassportBatch<PassportV1>>()?;
    Ok(batch.0.len())
}

fn solve_part_2(input: &str) -> Result<usize> {
    let batch = input.parse::<PassportBatch<PassportV2>>()?;
    Ok(batch.0.len())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PassportBatch<P>(Vec<P>);

impl<P> FromStr for PassportBatch<P>
where
    P: FromStr,
    anyhow::Error: From<P::Err>,
    P::Err: std::fmt::Display,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let passports = s
            .split("\n\n")
            // .enumerate()
            // .filter_map(|(i, chunk)| match chunk.parse::<P>() {
            //     Ok(p) => Some(p),
            //     Err(e) => {
            //         eprintln!("passport {i} is invalid: {e}");
            //         None
            //     }
            // })
            .flat_map(str::parse::<P>)
            .collect::<Vec<_>>();

        Ok(Self(passports))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PassportV1 {
    birth_year: String,
    issue_year: String,
    expiration_year: String,
    height: String,
    hair_color: String,
    eye_color: String,
    passport_id: String,
    country_id: String,
}

impl FromStr for PassportV1 {
    type Err = anyhow::Error;

    /// Parses "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
    /// byr:1937 iyr:2017 cid:147 hgt:183cm" to `Passport`.
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let fields: Vec<&str> = s.split_whitespace().collect();
        let (
            mut birth_year,
            mut issue_year,
            mut expiration_year,
            mut height,
            mut hair_color,
            mut eye_color,
            mut passport_id,
            mut country_id,
        ) = (
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
        );

        for field in fields {
            if let Some((field, value)) = field.split_once(":") {
                match field {
                    "byr" => birth_year = value.to_string(),
                    "iyr" => issue_year = value.to_string(),
                    "eyr" => expiration_year = value.to_string(),
                    "hgt" => height = value.to_string(),
                    "hcl" => hair_color = value.to_string(),
                    "ecl" => eye_color = value.to_string(),
                    "pid" => passport_id = value.to_string(),
                    "cid" => country_id = value.to_string(),
                    _ => {}
                }
            }
        }

        if !birth_year.is_empty()
            && !issue_year.is_empty()
            && !expiration_year.is_empty()
            && !height.is_empty()
            && !hair_color.is_empty()
            && !eye_color.is_empty()
            && !passport_id.is_empty()
        // && !country_id.is_empty()
        {
            Ok(Self {
                birth_year,
                issue_year,
                expiration_year,
                height,
                hair_color,
                eye_color,
                passport_id,
                country_id,
            })
        } else {
            Err(anyhow!("Invalid passport"))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PassportV2 {
    birth_year: u16,
    issue_year: u16,
    expiration_year: u16,
    height: Height,
    hair_color: HairColor,
    eye_color: EyeColor,
    passport_id: PassportId,
}

impl FromStr for PassportV2 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut byr = None;
        let mut iyr = None;
        let mut eyr = None;
        let mut hgt = None;
        let mut hcl = None;
        let mut ecl = None;
        let mut pid = None;

        for token in s.split_whitespace() {
            let (key, value) = token
                .split_once(':')
                .ok_or_else(|| anyhow!("invalid field: {token}"))?;

            match key {
                "byr" => {
                    let year = value.parse::<u16>()?;
                    ensure!(
                        (1920..=2002).contains(&year),
                        "birth year ({year}) out of bounds"
                    );
                    byr = Some(year);
                }
                "iyr" => {
                    let year = value.parse::<u16>()?;
                    ensure!(
                        (2010..=2020).contains(&year),
                        "issue year ({year}) out of bounds"
                    );
                    iyr = Some(year);
                }
                "eyr" => {
                    let year = value.parse::<u16>()?;
                    ensure!(
                        (2020..=2030).contains(&year),
                        "expiration year ({year}) out of bounds"
                    );
                    eyr = Some(year);
                }
                "hgt" => hgt = Some(value.parse::<Height>()?),
                "hcl" => hcl = Some(value.parse::<HairColor>()?),
                "ecl" => ecl = Some(value.parse::<EyeColor>()?),
                "pid" => pid = Some(value.parse::<PassportId>()?),
                "cid" => {}
                _ => bail!("unknown field: {token}"),
            }
        }

        Ok(Self {
            birth_year: byr.ok_or_else(|| anyhow!("missing byr"))?,
            issue_year: iyr.ok_or_else(|| anyhow!("missing iyr"))?,
            expiration_year: eyr.ok_or_else(|| anyhow!("missing eyr"))?,
            height: hgt.ok_or_else(|| anyhow!("missing hgt"))?,
            hair_color: hcl.ok_or_else(|| anyhow!("missing hcl"))?,
            eye_color: ecl.ok_or_else(|| anyhow!("missing ecl"))?,
            passport_id: pid.ok_or_else(|| anyhow!("missing pid"))?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Height {
    value: u8,
    unit: LengthUnit,
}

impl FromStr for Height {
    type Err = anyhow::Error;

    /// Parses "176cm" or ""
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let s = s.trim();
        let (value, unit) = s
            .split_at_checked(s.len() - 2)
            .ok_or_else(|| anyhow!("malformed height {s}"))?;

        let unit = unit.parse::<LengthUnit>()?;
        let value = value.parse::<u8>()?;

        ensure!(
            match unit {
                LengthUnit::Centimeter => (150..=193).contains(&value),
                LengthUnit::Inch => (59..=76).contains(&value),
            },
            "height ({s}) not within range"
        );

        Ok(Self { value, unit })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LengthUnit {
    Centimeter,
    Inch,
}

impl FromStr for LengthUnit {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.trim() {
            "cm" => Ok(Self::Centimeter),
            "in" => Ok(Self::Inch),
            _ => bail!("unknown unit: {s}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HairColor([u8; 3]);

impl FromStr for HairColor {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let s = s
            .strip_prefix('#')
            .ok_or_else(|| anyhow!("hair color ({s}) must start with '#'"))?;

        ensure!(
            s.len() == 6,
            "hex color ({s}) must be exactly six hexadecimals"
        );

        let r = u8::from_str_radix(&s[0..2], 16)?;
        let g = u8::from_str_radix(&s[2..4], 16)?;
        let b = u8::from_str_radix(&s[4..6], 16)?;

        Ok(Self([r, g, b]))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EyeColor {
    Amber,
    Blue,
    Brown,
    Gray,
    Green,
    Hazlenut,
    Other,
}

impl FromStr for EyeColor {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "amb" => Ok(Self::Amber),
            "blu" => Ok(Self::Blue),
            "brn" => Ok(Self::Brown),
            "gry" => Ok(Self::Gray),
            "grn" => Ok(Self::Green),
            "hzl" => Ok(Self::Hazlenut),
            "oth" => Ok(Self::Other),
            _ => bail!("unknown color: {s}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PassportId([u8; 9]);

impl FromStr for PassportId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        ensure!(
            bytes.iter().all(|s| s.is_ascii_digit()),
            "id ({bytes:?}) must consist only of ascii digits"
        );

        let id: [u8; 9] = bytes
            .try_into()
            .map_err(|_| anyhow!("id ({bytes:?}) must be exactly 9 digits"))?;

        Ok(Self(id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn test_solve_part_2() {
        let result = solve_part_2(EXAMPLE).unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_solve_part_1() {
        let result = solve_part_1(EXAMPLE).unwrap();
        assert_eq!(result, 2);
    }
}

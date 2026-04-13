use std::{collections::HashSet, fs, str::FromStr};

use anyhow::{Result, ensure};

fn main() -> Result<()> {
    let input: Input = fs::read_to_string("input.txt")?.parse()?;

    let sol1 = input.count_support_tls();
    println!("Part 1: {sol1}");

    let sol2 = input.count_support_ssl();
    println!("Part 2: {sol2}");

    Ok(())
}

struct Input(Vec<Ipv7>);

impl Input {
    fn count_support_tls(&self) -> usize {
        self.0.iter().filter(|ip| ip.supports_tls()).count()
    }

    fn count_support_ssl(&self) -> usize {
        self.0.iter().filter(|ip| ip.supports_ssl()).count()
    }
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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
struct Ipv7 {
    supernets: Vec<String>,
    hypernets: Vec<String>,
}

impl Ipv7 {
    fn supports_tls(&self) -> bool {
        self.supernets.iter().any(|s| contains_abba(s))
            && self.hypernets.iter().all(|s| !contains_abba(s))
    }

    fn supports_ssl(&self) -> bool {
        let abas: HashSet<[u8; 2]> = self.supernets.iter().flat_map(|s| extract_aba(s)).collect();

        self.hypernets
            .iter()
            .flat_map(|s| extract_aba(s))
            .any(|[a, b]| abas.contains(&[b, a]))
    }
}

fn extract_aba(s: &str) -> impl Iterator<Item = [u8; 2]> {
    s.as_bytes()
        .array_windows()
        .filter(|[a, b, c]| a == c && a != b)
        .map(|[a, b, _c]| [*a, *b])
}

fn contains_abba(s: &str) -> bool {
    s.as_bytes()
        .array_windows()
        .any(|[a, b, c, d]| a == d && b == c && a != b)
}

impl FromStr for Ipv7 {
    type Err = anyhow::Error;

    /// parses `abba[mnop]qrst` to:
    /// ```
    /// Ipv7 {
    ///     supernets: vec!["abba".to_owned(), "qrst".to_owned()],
    ///     hypernets: vec!["mnop".to_owned()],
    /// }
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let open_count = s.chars().filter(|&c| c == '[').count();
        let close_count = s.chars().filter(|&c| c == ']').count();

        ensure!(open_count == close_count, "unbalanced brackets");

        let mut supernets = Vec::with_capacity(open_count + 1);
        let mut hypernets = Vec::with_capacity(open_count);

        for (i, part) in s.split(['[', ']']).enumerate() {
            if i.is_multiple_of(2) {
                supernets.push(part.to_owned());
            } else {
                hypernets.push(part.to_owned());
            }
        }

        Ok(Self {
            supernets,
            hypernets,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supports_ssl() {
        assert!("aba[bab]xyz".parse::<Ipv7>().unwrap().supports_ssl());
        assert!(!"xyx[xyx]xyx".parse::<Ipv7>().unwrap().supports_ssl());
        assert!("zazbz[bzb]cdb".parse::<Ipv7>().unwrap().supports_ssl());
    }

    #[test]
    fn test_supports_tls() {
        assert!("abba[mnop]qrst".parse::<Ipv7>().unwrap().supports_tls());
        assert!(!"abcd[bddb]xyyx".parse::<Ipv7>().unwrap().supports_tls());
        assert!(!"aaaa[qwer]tyui".parse::<Ipv7>().unwrap().supports_tls());
        assert!(
            "ioxxoj[asdfgh]zxcvbn"
                .parse::<Ipv7>()
                .unwrap()
                .supports_tls()
        );
    }

    #[test]
    fn test_parse() {
        let result: Ipv7 = "abba[mnop]qrst".parse().unwrap();
        let expected = Ipv7 {
            supernets: vec!["abba".to_owned(), "qrst".to_owned()],
            hypernets: vec!["mnop".to_owned()],
        };
        assert_eq!(result, expected);
    }
}

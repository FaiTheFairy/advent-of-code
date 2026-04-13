use std::{collections::HashMap, fs, str::FromStr};

use anyhow::{Context, Result, ensure};

fn main() -> Result<()> {
    let input: Recording = fs::read_to_string("input.txt")?.parse()?;
    let sol1 = input
        .error_corrected(Correction::Unmodified)
        .context("no solution found for part 1")?;
    println!("Part 1: {}", sol1.as_str());

    let sol2 = input
        .error_corrected(Correction::Modified)
        .context("no solution found for part 2")?;
    println!("Part 2: {}", sol2.as_str());
    Ok(())
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Correction {
    Unmodified,
    Modified,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Recording(Vec<Message>);

impl Recording {
    fn width(&self) -> Option<usize> {
        self.0.first().map(Message::len)
    }

    fn error_corrected(&self, correction: Correction) -> Option<Message> {
        let width = self.width()?;
        let mut message = String::with_capacity(width);

        for col in 0..width {
            match correction {
                Correction::Modified => message.push(self.least_common_letter(col)?),
                Correction::Unmodified => message.push(self.most_common_letter(col)?),
            }
        }

        Some(Message(message))
    }

    fn most_common_letter(&self, col: usize) -> Option<char> {
        let count = self.count_chars(col);

        count
            .into_iter()
            .max_by_key(|&(_ch, n)| n)
            .map(|(ch, _n)| ch)
    }

    fn least_common_letter(&self, col: usize) -> Option<char> {
        let count = self.count_chars(col);

        count
            .into_iter()
            .min_by_key(|&(_ch, n)| n)
            .map(|(ch, _n)| ch)
    }

    fn count_chars(&self, col: usize) -> HashMap<char, usize> {
        let mut count: HashMap<char, usize> = HashMap::new();

        for ch in self.0.iter().filter_map(|message| message.char_at(col)) {
            *count.entry(ch).or_insert(0) += 1;
        }
        count
    }
}

impl FromStr for Recording {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner: Vec<Message> = s
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(|s| Message(s.to_string()))
            .collect();

        if let Some(width) = inner.first().map(Message::len) {
            ensure!(
                inner.iter().all(|message| message.len() == width),
                "all messages must have the same width"
            );
        }

        Ok(Self(inner))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Message(String);

impl Message {
    fn as_str(&self) -> &str {
        &self.0
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn char_at(&self, index: usize) -> Option<char> {
        self.0.as_bytes().get(index).copied().map(char::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar
";

    #[test]
    fn test_parse_recording() {
        let recording: Recording = EXAMPLE.parse().unwrap();

        assert_eq!(recording.0.len(), 16);
        assert_eq!(recording.0[0].as_str(), "eedadn");
        assert_eq!(recording.0[15].as_str(), "enarar");
    }

    #[test]
    fn test_most_common_letter_by_column() {
        let recording: Recording = EXAMPLE.parse().unwrap();

        assert_eq!(recording.most_common_letter(0), Some('e'));
        assert_eq!(recording.most_common_letter(1), Some('a'));
        assert_eq!(recording.most_common_letter(2), Some('s'));
        assert_eq!(recording.most_common_letter(3), Some('t'));
        assert_eq!(recording.most_common_letter(4), Some('e'));
        assert_eq!(recording.most_common_letter(5), Some('r'));
    }

    #[test]
    fn test_error_corrected_example_unmodified() {
        let recording: Recording = EXAMPLE.parse().unwrap();
        let message: Message = recording.error_corrected(Correction::Unmodified).unwrap();

        assert_eq!(message.as_str(), "easter");
    }

    #[test]
    fn test_error_corrected_example_modified() {
        let recording: Recording = EXAMPLE.parse().unwrap();
        let message: Message = recording.error_corrected(Correction::Modified).unwrap();

        assert_eq!(message.as_str(), "advent");
    }

    #[test]
    fn test_column_out_of_bounds_returns_none() {
        let recording: Recording = EXAMPLE.parse().unwrap();
        assert_eq!(recording.most_common_letter(6), None);
    }
}

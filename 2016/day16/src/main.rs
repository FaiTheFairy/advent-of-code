use std::str::FromStr;

use anyhow::{Result, ensure};

fn main() -> Result<()> {
    let mut disk = Disk::empty(272);
    let data: Data = "10011111011011001".parse()?;
    disk.set_data(data.clone());
    disk.expand();
    let sol1 = disk.data().checksum();
    println!("Part 1: {sol1}");

    let mut disk_2 = Disk::with_data(35651584, data);
    disk_2.expand();
    let sol2 = disk_2.data().checksum();
    println!("Part 2: {sol2}");

    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Disk {
    len: usize,
    data: Data,
}

impl Disk {
    fn empty(len: usize) -> Disk {
        Self {
            len,
            data: Data::default(),
        }
    }

    fn expand(&mut self) {
        while self.data.len() < self.len() {
            self.set_data(self.data.transformed());
        }

        self.data.0.truncate(self.len());
    }

    fn with_data(len: usize, data: Data) -> Self {
        Self { len, data }
    }

    fn len(&self) -> usize {
        self.len
    }

    fn data(&self) -> &Data {
        &self.data
    }

    fn set_data(&mut self, data: Data) {
        self.data = data;
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq)]
struct Data(Vec<u8>);

impl FromStr for Data {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let inner: Vec<u8> = s.trim().bytes().map(|b| b - b'0').collect();
        ensure!(
            inner.iter().all(|d| matches!(d, 0 | 1)),
            "`Data` only holds zeros and ones: {s}"
        );
        Ok(Self(inner))
    }
}

impl Data {
    fn checksum(&self) -> Self {
        let mut out = Data(Vec::with_capacity(self.0.len() / 2));

        for w in self.0.chunks_exact(2) {
            if w[0] == w[1] {
                out.push(1);
            } else {
                out.push(0);
            }
        }

        if !out.0.len().is_multiple_of(2) {
            return out;
        }

        out.checksum()
    }

    fn transform(&mut self) {
        *self = self.transformed();
    }

    fn transformed(&self) -> Self {
        let mut out = self.clone();
        out.push(0);
        for entry in self.0.iter().rev() {
            if *entry == 0 {
                out.push(1);
            } else {
                out.push(0);
            }
        }
        out
    }

    fn push(&mut self, value: u8) {
        self.0.push(value);
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in &self.0 {
            f.write_str(&i.to_string())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn data(s: &str) -> Data {
        s.parse().unwrap()
    }

    #[test]
    fn test_disk_part_1() {
        let disk = {
            let mut empty = Disk::with_data(20, "10000".parse().unwrap());
            empty.expand();
            empty
        };
        assert!(disk.len() == 20);
        assert_eq!(disk.data, data("10000011110010000111"));
        assert_eq!(disk.data.checksum(), data("01100"))
    }

    #[test]
    fn test_checksum() {
        let result = data("110010110100").checksum();
        let expected = data("100");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_transformed_data() {
        assert_eq!(data("1").transformed(), data("100"));
        assert_eq!(data("0").transformed(), data("001"));
        assert_eq!(data("11111").transformed(), data("11111000000"));
        assert_eq!(
            data("111100001010").transformed(),
            data("1111000010100101011110000")
        );
    }
}

use std::collections::HashMap;

const SALT: &str = "jlmsuwbz";
const STRETCHING_PART_2: usize = 2016;

fn main() {
    let mut key_gen = KeyGenerator::new(SALT, Stretching::None);
    let sol1 = key_gen.index_of_nth_key(64);
    println!("Part 1: {sol1}");

    let mut key_gen = KeyGenerator::new(SALT, Stretching::Value(STRETCHING_PART_2));
    let sol2 = key_gen.index_of_nth_key(64);
    println!("Part 2: {sol2}");
}

#[derive(Debug)]
struct KeyGenerator<'a> {
    salt: &'a str,
    cache: HashMap<usize, String>,
    stretching: Stretching,
}

impl<'a> KeyGenerator<'a> {
    fn new(salt: &'a str, stretching: Stretching) -> Self {
        Self {
            salt,
            cache: HashMap::new(),
            stretching,
        }
    }

    fn hash(&mut self, index: usize) -> &str {
        self.cache
            .entry(index)
            .or_insert_with(|| {
                let mut s = format!("{:x}", md5::compute(format!("{}{}", self.salt, index)));

                for _ in 0..self.stretching.value() {
                    s = format!("{:x}", md5::compute(s));
                }

                s
            })
            .as_str()
    }

    fn first_triplet(&mut self, index: usize) -> Option<u8> {
        let bytes = self.hash(index).as_bytes();

        for w in bytes.windows(3) {
            if w[0] == w[1] && w[1] == w[2] {
                return Some(w[0]);
            }
        }

        None
    }

    fn has_quintuple(&mut self, index: usize, needle: u8) -> bool {
        let bytes = self.hash(index).as_bytes();

        bytes.windows(5).any(|w| w.iter().all(|&b| b == needle))
    }

    fn is_key(&mut self, index: usize) -> bool {
        let Some(c) = self.first_triplet(index) else {
            return false;
        };

        for i in index + 1..=index + 1000 {
            if self.has_quintuple(i, c) {
                return true;
            }
        }

        false
    }

    fn index_of_nth_key(&mut self, n: usize) -> usize {
        let mut found = 0;
        let mut index = 0;

        loop {
            if self.is_key(index) {
                found += 1;

                if found == n {
                    return index;
                }
            }

            index += 1;
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Stretching {
    None,
    Value(usize),
}

impl Stretching {
    fn value(self) -> usize {
        match self {
            Stretching::None => 0,
            Stretching::Value(val) => val,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_2() {
        let mut key_gen = KeyGenerator::new("abc", Stretching::Value(STRETCHING_PART_2));
        let sol2 = key_gen.index_of_nth_key(64);
        assert_eq!(sol2, 22551);
    }

    #[test]
    fn test_example_part_1() {
        let mut key_gen = KeyGenerator::new("abc", Stretching::None);
        let sol1 = key_gen.index_of_nth_key(64);
        assert_eq!(sol1, 22728);
    }
}

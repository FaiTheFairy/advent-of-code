use std::{fs, str::FromStr};

use anyhow::Result;

fn main() -> Result<()> {
    let numbers: Numbers = fs::read_to_string("input.txt")?.parse()?;
    let sol1 = numbers.captcha_sum_v1();
    println!("Part 1: {sol1}");

    let sol2 = numbers.captcha_sum_v2();
    println!("Part 2: {sol2}");
    Ok(())
}

struct Numbers(Vec<u8>);

impl Numbers {
    fn captcha_sum_v2(&self) -> u32 {
        let len = self.0.len();

        (0..len)
            .filter(|i| self.0[*i] == self.0[(i + len / 2) % len])
            .map(|i| u32::from(self.0[i]))
            .sum()
    }

    fn captcha_sum_v1(&self) -> u32 {
        let len = self.0.len();

        (0..len)
            .filter(|i| self.0[*i] == self.0[(i + 1) % len])
            .map(|i| u32::from(self.0[i]))
            .sum()
    }
}

impl FromStr for Numbers {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner: Vec<u8> = s
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(u8::try_from)
            .collect::<Result<_, _>>()?;

        Ok(Self(inner))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn num(s: &str) -> Numbers {
        s.parse().unwrap()
    }

    #[test]
    fn test_captcha_sum_v2() {
        assert_eq!(num("1212").captcha_sum_v2(), 6);
        assert_eq!(num("1221").captcha_sum_v2(), 0);
        assert_eq!(num("123425").captcha_sum_v2(), 4);
        assert_eq!(num("123123").captcha_sum_v2(), 12);
        assert_eq!(num("12131415").captcha_sum_v2(), 4);
    }

    #[test]
    fn test_captcha_sum_v1() {
        assert_eq!(num("1122").captcha_sum_v1(), 3);
        assert_eq!(num("1111").captcha_sum_v1(), 4);
        assert_eq!(num("1234").captcha_sum_v1(), 0);
        assert_eq!(num("91212129").captcha_sum_v1(), 9);
    }
}

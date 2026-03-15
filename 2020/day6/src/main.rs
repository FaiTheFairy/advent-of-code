use std::{
    collections::{HashMap, HashSet},
    fs,
    str::FromStr,
};

use anyhow::Result;

fn main() -> Result<()> {
    let list = fs::read_to_string("input.txt")?.parse::<GroupsList>()?;
    let sol1 = list.sum_yes_answers_part_1();
    println!("Part 1. sum of counts of 'yes' answers = {sol1}");

    let sol2 = list.sum_yes_answers_part_2();
    println!("Part 2. sum of counts where everyone in a group answered 'yes' = {sol2}");

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GroupsList(Vec<Group>);

impl GroupsList {
    fn sum_yes_answers_part_1(&self) -> usize {
        self.0.iter().map(Group::count_unique).sum()
    }

    fn sum_yes_answers_part_2(&self) -> usize {
        self.0.iter().map(Group::count_all_yes).sum()
    }
}

impl FromStr for GroupsList {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut out = Vec::new();
        for group in s.split("\n\n") {
            let group = group.parse::<Group>()?;
            out.push(group);
        }

        Ok(Self(out))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Group(Vec<Person>);

impl Group {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn count_unique(&self) -> usize {
        let mut set = HashSet::new();
        for person in &self.0 {
            for char in &person.0 {
                set.insert(char);
            }
        }

        set.len()
    }

    fn count_all_yes(&self) -> usize {
        let mut map = HashMap::new();
        for person in &self.0 {
            for char in &person.0 {
                map.entry(char).and_modify(|e| *e += 1).or_insert(1);
            }
        }

        map.iter()
            .filter(|(ch, count)| **count == self.len())
            .count()
    }
}

impl FromStr for Group {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut set = Vec::new();
        for line in s.lines() {
            let person = line.parse::<Person>()?;
            set.push(person);
        }

        Ok(Self(set))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Person(Vec<char>);

impl FromStr for Person {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let yes_answers: Vec<char> = s.trim().chars().collect();
        Ok(Self(yes_answers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn test_sum_yes_part_2() {
        let result = EXAMPLE
            .parse::<GroupsList>()
            .unwrap()
            .sum_yes_answers_part_2();
        assert_eq!(result, 6);
    }

    #[test]
    fn test_sum_yes() {
        let result = EXAMPLE
            .parse::<GroupsList>()
            .unwrap()
            .sum_yes_answers_part_1();
        assert_eq!(result, 11);
    }

    #[test]
    fn test_parse_list() {
        let result = EXAMPLE.parse::<GroupsList>().unwrap();
        let expected = GroupsList(vec![
            Group(vec![Person(vec!['a', 'b', 'c'])]),
            Group(vec![
                Person(vec!['a']),
                Person(vec!['b']),
                Person(vec!['c']),
            ]),
            Group(vec![Person(vec!['a', 'b']), Person(vec!['a', 'c'])]),
            Group(vec![Person(vec!['a']); 4]),
            Group(vec![Person(vec!['b'])]),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_person() {
        let result = "ab".parse::<Person>().unwrap();
        let expected = Person(vec!['a', 'b']);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_group() {
        let result = "ab\nac".parse::<Group>().unwrap();
        let expected = Group(vec![Person(vec!['a', 'b']), Person(vec!['a', 'c'])]);
        assert_eq!(result, expected);
    }
}

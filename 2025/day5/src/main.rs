use std::{
    collections::HashSet,
    fs::{self},
    path::PathBuf,
};

fn parse_input(input: &str) -> (HashSet<usize>, HashSet<usize>) {
    let mut fresh_ingredients: HashSet<usize> = HashSet::new();
    let mut available_ingredients: HashSet<usize> = HashSet::new();
    let (fresh_ids, available_ids) = input.split_once("\n\n").unwrap();
    for line in fresh_ids.lines() {
        let (first, last) = line.split_once('-').unwrap();
        let first: usize = first.parse().unwrap();
        let last: usize = last.parse().unwrap();
        for i in first..=last {
            // if !fresh_ingredients.contains(&i) {
            // fresh_ingredients.push(i);
            fresh_ingredients.insert(i);
            // }
        }
    }
    for line in available_ids.lines() {
        available_ingredients.insert(line.parse().unwrap());
    }

    (fresh_ingredients, available_ingredients)
}

fn main() {
    // let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("input1.txt");
    let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let input = fs::read_to_string(filename).unwrap();
    let (fresh, available) = parse_input(&input);
    println!("Fresh: {fresh:?}");
    println!("Available: {available:?}");
    // let mut count_fresh_available: usize = 0;
    // for ingredient in available {
    //     if fresh.contains(&ingredient) {
    //         count_fresh_available += 1;
    //     }
    // }
    let count_fresh_available: usize = available.intersection(&fresh).count();
    println!("Number of available ingredient IDs that are fresh = {count_fresh_available}");
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_parse_input() {
        let result: (Vec<usize>, Vec<usize>) = parse_input(INPUT);
        let expected_fresh: Vec<usize> = vec![3, 4, 5, 10, 11, 12, 13, 14, 16, 17, 18, 19, 20, 15];
        let expected_available: Vec<usize> = vec![1, 5, 8, 11, 17, 32];
        let expected = (expected_fresh, expected_available);
        assert_eq!(result, expected);
    }
}

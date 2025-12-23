use std::{
    // collections::HashSet,
    fs::{self},
    path::PathBuf,
};

fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<usize>) {
    let mut fresh_ingredients_ranges: Vec<(usize, usize)> = vec![];
    let mut available_ingredients: Vec<usize> = vec![];
    let (fresh_ids, available_ids) = input.split_once("\n\n").unwrap();
    for line in fresh_ids.lines() {
        let (first, last) = line.split_once('-').unwrap();
        let first: usize = first.parse().unwrap();
        let last: usize = last.parse().unwrap();
        fresh_ingredients_ranges.push((first, last));
    }
    for line in available_ids.lines() {
        available_ingredients.push(line.parse().unwrap());
    }

    (fresh_ingredients_ranges, available_ingredients)
}

fn main() {
    let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("input1.txt");
    // let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let input = fs::read_to_string(filename).unwrap();
    let (fresh_ranges, available_ids) = parse_input(&input);
    println!("Fresh: {fresh_ranges:?}");
    println!("Available: {available_ids:?}");
    // let mut count_fresh_available: usize = 0;
    // for ingredient in available {
    //     if fresh.contains(&ingredient) {
    //         count_fresh_available += 1;
    //     }
    // }
    let mut count_fresh_available: usize = 0;

    for id in available_ids {
        if fresh_ranges.iter().any(|(a, b)| (a..=b).contains(&&id)) {
            count_fresh_available += 1;
        }
    }
    // let count_fresh_available: usize = available_ids.intersection(&fresh_ranges).count();
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

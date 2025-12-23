use std::{
    // collections::HashSet,
    fs::{self},
    path::PathBuf,
};

/// Takes input that consists of a list of fresh ingredient ID ranges, a blank line, and a list of available ingredient IDs.
/// # Example
/// ```
/// let input = "3-5
///10-14
///16-20
///12-18
///
///1
///5
///8
///11
///17
///32";
///
/// let result: (Vec<(usize, usize)>, Vec<usize>) = parse_input(input);
/// let expected_fresh: Vec<(usize, usize)> = vec![(3, 5), (10, 14), (16, 20), (12, 18)];
/// let expected_available: Vec<usize> = vec![1, 5, 8, 11, 17, 32];
/// let expected = (expected_fresh, expected_available);
/// assert_eq!(result, expected);
/// ```
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

/// Given a reference to a vector of ranges organized in tuples (start, end),
/// this function returns a new vector of ranges that has no overlap
/// # Example
/// ```
/// let raw_ranges = vec![(1, 3), (5, 19), (17, 20), (19, 23)];
/// let merged_ranges = merge_ranges(&raw_ranges);
/// let expected = vec![(1, 3), (5, 23)];
/// assert_eq!(merged_ranges, expected);
/// ```
fn merge_ranges(raw_ranges: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut merged_ranges: Vec<(usize, usize)> = vec![];
    let mut sorted_ranges = raw_ranges.to_owned();

    // sort ranges in ascending order of start of each range
    sorted_ranges.sort_unstable_by_key(|(start1, _end1)| *start1);

    let &(mut start, mut end) = &sorted_ranges[0];
    for &(next_start, next_end) in &sorted_ranges {
        // check if next_start..next_end has overlap with start..end
        if next_start <= end {
            // extend end
            end = end.max(next_end);
        } else {
            merged_ranges.push((start, end));
            start = next_start;
            end = next_end;
        }
    }
    // push last range
    merged_ranges.push((start, end));
    merged_ranges
}

fn main() {
    // let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("input1.txt");
    let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let input = fs::read_to_string(filename).unwrap();
    let (fresh_ranges, available_ids) = parse_input(&input);
    // println!("Fresh: {fresh_ranges:?}");
    // println!("Available: {available_ids:?}");
    let mut count_fresh_available: usize = 0;
    let fresh_ranges_merged = merge_ranges(&fresh_ranges);

    for (start, end) in fresh_ranges_merged {
        for id in &available_ids {
            if (start..=end).contains(id) {
                count_fresh_available += 1;
            }
        }
    }
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
        let result: (Vec<(usize, usize)>, Vec<usize>) = parse_input(INPUT);
        let expected_fresh: Vec<(usize, usize)> = vec![(3, 5), (10, 14), (16, 20), (12, 18)];
        let expected_available: Vec<usize> = vec![1, 5, 8, 11, 17, 32];
        let expected = (expected_fresh, expected_available);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_merge_ranges() {
        let raw_ranges = vec![(1, 3), (5, 19), (17, 20), (19, 23)];
        let merged_ranges = merge_ranges(&raw_ranges);
        let expected = vec![(1, 3), (5, 23)];
        assert_eq!(merged_ranges, expected);
    }
}

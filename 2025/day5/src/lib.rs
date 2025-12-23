/// Parses input containing fresh ID ranges, a blank line, and available ingredient IDs.
///
/// # Example
/// ```
/// use day5::parse_input;
///
/// let (ranges, ids) = parse_input("1-2\n\n5\n6\n");
/// assert_eq!(ranges, vec![(1, 2)]);
/// assert_eq!(ids, vec![5, 6]);
/// ```
pub fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<usize>) {
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

/// Merges overlapping `(start, end)` ranges into a non-overlapping set.
///
/// The returned ranges are sorted by start value.
/// Ranges that touch at the boundary (e.g. `5..7` and `8..10`) are merged.
///
/// # Example
/// ```
/// use day5::merge_ranges;
///
/// let ranges = vec![(1, 3), (5, 19), (17, 20), (19, 23)];
/// assert_eq!(merge_ranges(&ranges), vec![(1, 3), (5, 23)]);
/// ```
pub fn merge_ranges(raw_ranges: &[(usize, usize)]) -> Vec<(usize, usize)> {
    if raw_ranges.is_empty() {
        return Vec::new();
    }

    let mut merged_ranges: Vec<(usize, usize)> = vec![];
    let mut sorted_ranges = raw_ranges.to_owned();

    // sort ranges in ascending order of start of each range
    sorted_ranges.sort_unstable_by_key(|(start, _end)| *start);

    let &(mut start, mut end) = &sorted_ranges[0];
    for &(next_start, next_end) in &sorted_ranges[1..] {
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

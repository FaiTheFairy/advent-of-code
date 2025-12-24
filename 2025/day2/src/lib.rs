pub fn split_string_at_comma(input: String) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    for value in input.split(',') {
        result.push(value.to_string());
    }
    result
}

pub fn parse_ranges_to_vec(ranges: Vec<String>) -> Vec<u64> {
    let mut result: Vec<u64> = vec![];
    for range in ranges {
        if let Some((first, last)) = range.split_once('-') {
            let first: u64 = first.parse().unwrap();
            let last: u64 = last.parse().unwrap();
            for i in first..=last {
                result.push(i);
            }
        }
    }
    result
}

pub fn extract_invalid_ids_repeated_twice(ids: Vec<u64>) -> Vec<u64> {
    let mut invalid_ids = vec![];
    for id in ids {
        if digits_repeated_exactly_twice(id) {
            invalid_ids.push(id);
        }
    }
    invalid_ids
}

pub fn extract_invalid_ids_repeated_twice_or_more(ids: Vec<u64>) -> Vec<u64> {
    let mut invalid_ids = vec![];
    for id in ids {
        if digits_repeated_at_least_twice(id) {
            invalid_ids.push(id);
        }
    }
    invalid_ids
}

pub fn digits_repeated_exactly_twice(id: u64) -> bool {
    let id_str = id.to_string();
    let len = id_str.chars().count();
    if !len.is_multiple_of(2) {
        return false;
    }
    let (first, last) = id_str.split_at(len / 2);
    first == last
}

pub fn digits_repeated_at_least_twice(id: u64) -> bool {
    let id_str = id.to_string();
    let len = id_str.len();

    // Must repeat at least twice, so period must be <= n/2
    for period in 1..=len / 2 {
        if !len.is_multiple_of(period) {
            continue;
        }
        let number_of_repeats = len / period;
        let unit = &id_str[..period];

        // Compare id_str with unit repeated number_of_repeats times
        if unit.repeat(number_of_repeats) == id_str {
            return true;
        }
    }

    false
}

pub fn sum_of_ids_repeated_exactly_twice(input: String) -> u64 {
    let ranges: Vec<String> = split_string_at_comma(input);
    let ids: Vec<u64> = parse_ranges_to_vec(ranges);
    let invalid_ids: Vec<u64> = extract_invalid_ids_repeated_twice(ids);
    invalid_ids.iter().sum()
}

pub fn sum_of_ids_repeated_at_least_twice(input: String) -> u64 {
    let ranges: Vec<String> = split_string_at_comma(input);
    let ids: Vec<u64> = parse_ranges_to_vec(ranges);
    let invalid_ids: Vec<u64> = extract_invalid_ids_repeated_twice_or_more(ids);
    invalid_ids.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn part1_repeated_exactly_twice() {
        let sum_invalid_ids: u64 = sum_of_ids_repeated_exactly_twice(SAMPLE.to_string());
        let expected = 1227775554u64;
        assert_eq!(sum_invalid_ids, expected);
    }

    #[test]
    fn part2_repeated_at_least_twice() {
        let sum_invalid_ids: u64 = sum_of_ids_repeated_at_least_twice(SAMPLE.to_string());
        let expected = 4174379265u64;
        assert_eq!(sum_invalid_ids, expected);
    }
}

fn split_string_at_comma(input: String) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    for value in input.split(',') {
        result.push(value.to_string());
    }
    result
}

fn parse_ranges_to_vec(ranges: Vec<String>) -> Vec<u64> {
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

fn extract_invalid_ids_repeated_twice(ids: Vec<u64>) -> Vec<u64> {
    let mut invalid_ids = vec![];
    for id in ids {
        if digits_repeated_exactly_twice(id) {
            invalid_ids.push(id);
        }
    }
    invalid_ids
}

fn extract_invalid_ids_repeated_twice_or_more(ids: Vec<u64>) -> Vec<u64> {
    let mut invalid_ids = vec![];
    for id in ids {
        if digits_repeated_at_least_twice(id) {
            invalid_ids.push(id);
        }
    }
    invalid_ids
}

fn digits_repeated_exactly_twice(id: u64) -> bool {
    let id_str = id.to_string();
    let len = id_str.chars().count();
    if len % 2 != 0 {
        return false;
    }
    let (first, last) = id_str.split_at(len / 2);
    if first == last { true } else { false }
}

pub fn digits_repeated_at_least_twice(id: u64) -> bool {
    let id_str = id.to_string();
    let len = id_str.len();

    // Must repeat at least twice, so period must be <= n/2
    for period in 1..=len / 2 {
        if len % period != 0 {
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

fn sum_of_ids_repeated_exactly_twice(input: String) -> u64 {
    let ranges: Vec<String> = split_string_at_comma(input);
    let ids: Vec<u64> = parse_ranges_to_vec(ranges);
    let invalid_ids: Vec<u64> = extract_invalid_ids_repeated_twice(ids);
    invalid_ids.iter().sum()
}

fn sum_of_ids_repeated_at_least_twice(input: String) -> u64 {
    let ranges: Vec<String> = split_string_at_comma(input);
    let ids: Vec<u64> = parse_ranges_to_vec(ranges);
    let invalid_ids: Vec<u64> = extract_invalid_ids_repeated_twice_or_more(ids);
    invalid_ids.iter().sum()
}

fn main() {
    let input = "1090286-1131879,3259566-3404881,138124-175118,266204727-266361099,16765-24272,7657360692-7657593676,88857504-88926597,6869078-6903096,48444999-48532270,61427792-61580535,71-103,8077-10421,1920-2560,2-17,951-1259,34-50,28994-36978,1309-1822,9393918461-9393960770,89479-120899,834641-988077,5389718924-5389797353,34010076-34214499,5063-7100,607034-753348,19098586-19261191,125085556-125188689,39839-51927,3246-5037,174-260,439715-473176,187287-262190,348-535,58956-78301,4388160-4505757,512092-584994,13388753-13534387".to_string();
    // println!("Input: {:?}", input);
    // let ranges: Vec<String> = split_string_at_comma(input);
    // // println!("Ranges: {:?}", ranges.clone());
    // let ids: Vec<u64> = parse_ranges_to_vec(ranges);
    // // println!("IDs: {:?}", ids.clone());
    // let invalid_ids: Vec<u64> = extract_invalid_ids_repeated_twice(ids);
    // // println!("Invalid IDs: {:?}", invalid_ids);
    let sum_invalid_ids_twice: u64 = sum_of_ids_repeated_exactly_twice(input.clone());
    println!(
        "Sum of IDs that include digits repeated exactly twice: {:?}",
        sum_invalid_ids_twice
    );
    let sum_invalid_ids_at_least_twice: u64 = sum_of_ids_repeated_at_least_twice(input);
    println!(
        "Sum of IDs that include digits repeated at least twice: {:?}",
        sum_invalid_ids_at_least_twice
    );
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

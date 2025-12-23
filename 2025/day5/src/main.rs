use std::{
    fs::{self},
    path::PathBuf,
};

use day5::{merge_ranges, parse_input};

fn main() {
    let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let input = fs::read_to_string(filename).unwrap();
    let (fresh_ranges, available_ids) = parse_input(&input);
    let mut count_fresh_available = 0usize;
    let mut count_fresh_total = 0usize;
    let fresh_ranges_merged = merge_ranges(&fresh_ranges);

    for (start, end) in fresh_ranges_merged {
        // check each id if it is within this range
        for id in &available_ids {
            if (start..=end).contains(id) {
                count_fresh_available += 1;
            }
        }

        // add number of fresh ingredients in this range to the count
        count_fresh_total += end - start + 1;
    }
    println!("Part 1. Number of available ingredient IDs that are fresh = {count_fresh_available}");
    println!("Part 2. Total number of fresh ingredients = {count_fresh_total}");
}

use std::{collections::HashSet, fs::File, io::Read, path::PathBuf};

type Coord = (usize, usize);

fn parse_grid(input: &str) -> HashSet<Coord> {
    let mut occupied = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '@' {
                occupied.insert((x, y));
            }
        }
    }

    occupied
}

fn count_adjacent_occupied(x: usize, y: usize, occupied: &HashSet<Coord>) -> usize {
    let mut count = 0;

    for dy in [-1i32, 0, 1] {
        for dx in [-1i32, 0, 1] {
            if dx == 0 && dy == 0 {
                continue;
            }

            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;

            // make sure coords we're checking are positive, since negative implies out
            // of bounds.
            if new_x >= 0 && new_y >= 0 {
                let coord = (new_x as usize, new_y as usize);
                if occupied.contains(&coord) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn count_accessible_rolls(occupied: &HashSet<Coord>) -> usize {
    occupied
        .iter()
        .filter(|&&(x, y)| count_adjacent_occupied(x, y, occupied) < 4)
        .count()
}

fn remove_accessible_rolls(occupied: &mut HashSet<Coord>) -> () {
    let accessible: Vec<Coord> = occupied
        .iter()
        .copied()
        .filter(|&(x, y)| count_adjacent_occupied(x, y, occupied) < 4)
        .collect();
    for coord in accessible {
        occupied.remove(&coord);
    }
}

fn main() {
    // open file and parse occupied positions into grid
    let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let mut buffer = String::new();
    File::open(filename)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let mut occupied = parse_grid(&buffer);

    // recursively finds accessible paper rolls and removes them, while counting how many have been removed
    let mut count_accessible = count_accessible_rolls(&occupied);
    println!("Part 1. Accesible paper rolls = {count_accessible}");

    while count_accessible_rolls(&occupied) != 0 {
        remove_accessible_rolls(&mut occupied);
        count_accessible += count_accessible_rolls(&occupied);
    }

    println!(
        "Part 2. Total accessible points (includes ones accessible only after removing first): {count_accessible}"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn parse_counts_occupied_cells() {
        let occupied = parse_grid(SAMPLE);
        // should be less than 100 and > 0
        assert!(occupied.len() > 0);
        assert!(occupied.len() < 100);
    }

    #[test]
    fn adjacent_count_ignores_self() {
        let occupied = parse_grid("@");
        assert_eq!(count_adjacent_occupied(0, 0, &occupied), 0);
    }

    #[test]
    fn adjacent_count_uses_eight_neighbors() {
        let occupied = parse_grid(
            "@@@
@@@
@@@",
        );
        // center has 8 neighbors occupied
        assert_eq!(count_adjacent_occupied(1, 1, &occupied), 8);
        // corner has 3 neighbors occupied
        assert_eq!(count_adjacent_occupied(0, 0, &occupied), 3);
    }

    #[test]
    fn accessible_rolls_in_sample_grid() {
        let occupied = parse_grid(SAMPLE);
        let accessible = count_accessible_rolls(&occupied);

        assert_eq!(accessible, 13);
    }
}

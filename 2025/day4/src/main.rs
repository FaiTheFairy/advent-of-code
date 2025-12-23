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

            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if nx >= 0 && ny >= 0 {
                let coord = (nx as usize, ny as usize);
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
    let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let mut buffer = String::new();
    File::open(filename)
        .expect("Cannot open file")
        .read_to_string(&mut buffer)
        .unwrap();

    let mut occupied = parse_grid(&buffer);

    let mut count_accessible = count_accessible_rolls(&occupied);
    while count_accessible_rolls(&occupied) != 0 {
        remove_accessible_rolls(&mut occupied);
        count_accessible += count_accessible_rolls(&occupied);
    }

    println!(
        "Total accessible points (includes ones accessible only after removing first): {count_accessible}"
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
        let occ = parse_grid(SAMPLE);
        // sanity check: should be less than 100 and > 0
        assert!(occ.len() > 0);
        assert!(occ.len() < 100);
    }

    #[test]
    fn adjacent_count_ignores_self() {
        let occ = parse_grid("@");
        assert_eq!(count_adjacent_occupied(0, 0, &occ), 0);
    }

    #[test]
    fn adjacent_count_uses_eight_neighbors() {
        let occ = parse_grid(
            "@@@
@@@
@@@",
        );
        // center has 8 neighbors occupied
        assert_eq!(count_adjacent_occupied(1, 1, &occ), 8);
        // corner has 3 neighbors occupied
        assert_eq!(count_adjacent_occupied(0, 0, &occ), 3);
    }

    #[test]
    fn accessible_rolls_in_sample_grid() {
        let occ = parse_grid(SAMPLE);
        let accessible = count_accessible_rolls(&occ);

        // This is the number you said you expect for this sample.
        assert_eq!(accessible, 13);
    }
}

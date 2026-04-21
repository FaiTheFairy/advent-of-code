use std::collections::HashSet;

fn main() {
    let key = "amgozmfv";

    let used = used_squares(key);
    let sol1 = used.len();
    println!("Part 1: {sol1}");

    let sol2 = region_count(&used);
    println!("Part 2: {sol2}");
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn neighbors(self) -> [Option<Self>; 4] {
        let up = self
            .row
            .checked_sub(1)
            .map(|row| Self { row, col: self.col });
        let down = (self.row < 127).then_some(Self {
            row: self.row + 1,
            col: self.col,
        });
        let left = self
            .col
            .checked_sub(1)
            .map(|col| Self { row: self.row, col });
        let right = (self.col < 127).then_some(Self {
            row: self.row,
            col: self.col + 1,
        });

        [up, down, left, right]
    }
}

fn used_squares(key: &str) -> HashSet<Coord> {
    let mut used = HashSet::new();

    for row in 0..128 {
        let input = format!("{key}-{row}");
        for (col, bit) in hash_bits(&input).enumerate() {
            if bit {
                used.insert(Coord { row, col });
            }
        }
    }

    used
}

fn region_count(used: &HashSet<Coord>) -> usize {
    let mut remaining = used.clone();
    let mut count = 0;

    while let Some(&start) = remaining.iter().next() {
        count += 1;
        let mut stack = vec![start];

        while let Some(coord) = stack.pop() {
            if !remaining.remove(&coord) {
                continue;
            }

            for neighbor in coord.neighbors().into_iter().flatten() {
                if remaining.contains(&neighbor) {
                    stack.push(neighbor);
                }
            }
        }
    }

    count
}

fn hash_bits(input: &str) -> impl Iterator<Item = bool> {
    knot_hash(input)
        .into_iter()
        .flat_map(|byte| (0..8).rev().map(move |bit| ((byte >> bit) & 1) == 1))
}

fn knot_hash(input: &str) -> [u8; 16] {
    let mut list: Vec<u8> = (u8::MIN..=u8::MAX).collect();
    let mut position = 0usize;
    let mut skip_size = 0usize;

    let mut lengths: Vec<usize> = input.bytes().map(usize::from).collect();
    lengths.extend([17usize, 31, 73, 47, 23]);

    for _ in 0..64 {
        for &length in &lengths {
            reverse_circular(&mut list, position, length);
            position = (position + length + skip_size) % list.len();
            skip_size += 1;
        }
    }

    let mut dense = [0u8; 16];
    for (i, chunk) in list.chunks_exact(16).enumerate() {
        dense[i] = chunk.iter().copied().reduce(|acc, x| acc ^ x).unwrap();
    }

    dense
}

fn reverse_circular(list: &mut [u8], start: usize, length: usize) {
    let n = list.len();
    for offset in 0..(length / 2) {
        let a = (start + offset) % n;
        let b = (start + length - 1 - offset) % n;
        list.swap(a, b);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_1() {
        let used = used_squares("flqrgnkx");
        assert_eq!(used.len(), 8108);
    }

    #[test]
    fn test_example_part_2() {
        let used = used_squares("flqrgnkx");
        assert_eq!(region_count(&used), 1242);
    }
}

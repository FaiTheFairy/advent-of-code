use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let sum1 = sum_part_1(&input);
    println!("Part 1. sum = {sum1}");
    let sum2 = sum_part_2(&input);
    println!("Part 2. sum = {sum2}");
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid {
    w: usize,
    h: usize,
    cells: Vec<u8>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let h = lines.len();
        let w = lines.first().unwrap().len();

        let mut cells = Vec::with_capacity(w * h);

        for line in lines {
            cells.extend_from_slice(line.as_bytes());
        }

        Self { w, h, cells }
    }
    fn at(&self, r: usize, c: usize) -> u8 {
        self.cells[r * self.w + c]
    }

    fn neighbors8(&self, r: usize, c: usize) -> impl Iterator<Item = (usize, usize)> {
        // let r0 = r.saturating_sub(1);
        // let r1 = (r + 1).min(self.h - 1);
        // let c0 = c.saturating_sub(1);
        // let c1 = (c + 1).min(self.w - 1);

        // (r0..=r1).flat_map(move |rr| {
        //     (c0..=c1).filter_map(move |cc| (rr != r || cc != c).then_some((rr, cc)))
        // })
        self.box_around_span(r, c, c)
    }

    fn box_around_span(
        &self,
        r: usize,
        start: usize,
        end: usize,
    ) -> impl Iterator<Item = (usize, usize)> {
        let r0 = r.saturating_sub(1);
        let r1 = (r + 1).min(self.h - 1);

        let c0 = start.saturating_sub(1);
        let c1 = end.min(self.w - 1);

        (r0..=r1).flat_map(move |rr| (c0..=c1).map(move |cc| (rr, cc)))
    }

    fn scan_number_at(&self, r: usize, c: usize) -> Option<(usize, usize, u32)> {
        let b = self.at(r, c);
        if !b.is_ascii_digit() {
            return None;
        }

        let start = c;
        let mut cc = c;
        let mut value = 0u32;

        while cc < self.w {
            let d = self.at(r, cc);
            if !d.is_ascii_digit() {
                break;
            }
            value = value * 10 + (d - b'0') as u32;
            cc += 1;
        }

        let end = cc; // exclusive
        Some((start, end, value))
    }

    fn has_adjacent_symbol(&self, r: usize, start: usize, end: usize) -> bool {
        // end is exclusive
        for c in start..end {
            if self
                .neighbors8(r, c)
                .any(|(rr, cc)| is_symbol(self.at(rr, cc)))
            {
                return true;
            }
        }
        false
    }
}

fn sum_part_1(input: &str) -> u32 {
    let grid = Grid::new(input);
    let mut sum: u32 = 0;

    for r in 0..grid.h {
        let mut c = 0;
        while c < grid.w {
            if let Some((start, end, value)) = grid.scan_number_at(r, c) {
                if grid.has_adjacent_symbol(r, start, end) {
                    sum += value;
                }
                c = end; // skip past the whole scanned number
            } else {
                c += 1;
            }
        }
    }
    sum
}

fn sum_part_2(input: &str) -> u32 {
    let grid = Grid::new(input);

    let mut stars: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    for r in 0..grid.h {
        let mut c = 0;
        while c < grid.w {
            if let Some((start, end, value)) = grid.scan_number_at(r, c) {
                // record adjacency to any '*'
                for (rr, cc) in grid.box_around_span(r, start, end) {
                    if grid.at(rr, cc) == b'*' {
                        stars.entry((rr, cc)).or_default().push(value);
                    }
                }

                c = end; // skip past this number
            } else {
                c += 1;
            }
        }
    }
    stars
        .values()
        .filter_map(|nums| {
            if nums.len() == 2 {
                Some(nums[0] * nums[1])
            } else {
                None
            }
        })
        .sum()
}

fn is_symbol(b: u8) -> bool {
    !b.is_ascii_digit() && b != b'.'
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    #[test]
    fn test_part_1() {
        assert_eq!(sum_part_1(EXAMPLE), 4361u32);
    }

    #[test]
    fn test_parse_to_grid() {
        let input = "..+\n.*.\n12.";
        assert_eq!(
            Grid::new(input),
            Grid {
                w: 3,
                h: 3,
                cells: vec![b'.', b'.', b'+', b'.', b'*', b'.', b'1', b'2', b'.']
            }
        )
    }
}

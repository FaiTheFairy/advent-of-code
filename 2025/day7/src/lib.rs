use std::{
    collections::{HashMap, HashSet},
    mem::swap,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Cell {
    Source,
    Splitter,
    Empty,
}

pub fn parse_input(input: &str) -> Vec<Vec<Cell>> {
    let mut grid: Vec<Vec<Cell>> = vec![];
    for line in input.lines() {
        let mut row: Vec<Cell> = vec![];
        for char in line.chars() {
            match char {
                'S' => row.push(Cell::Source),
                '^' => row.push(Cell::Splitter),
                '.' => row.push(Cell::Empty),
                _ => panic!("Unexpected char!"),
            }
        }
        grid.push(row);
    }
    grid
}

pub fn count_splits(grid: &[Vec<Cell>]) -> usize {
    let rows = grid.len();
    let cols = grid.first().map(|r| r.len()).unwrap_or(0);

    let mut active: HashSet<(usize, usize)> = HashSet::new();
    let mut next: HashSet<(usize, usize)> = HashSet::new();
    let mut split_count: usize = 0;

    // seed at S
    for (r, row) in grid.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            if *cell == Cell::Source {
                active.insert((r, c));
            }
        }
    }

    while !active.is_empty() {
        next.clear();

        for (r, c) in active.iter().copied() {
            let r1 = r + 1;
            if r1 >= rows {
                continue;
            }

            match grid[r][c] {
                Cell::Splitter => {
                    split_count += 1;

                    if c > 0 {
                        next.insert((r1, c - 1));
                    }
                    if c + 1 < cols {
                        next.insert((r1, c + 1));
                    }
                }
                _ => {
                    next.insert((r1, c));
                }
            }
        }

        swap(&mut active, &mut next);
    }

    split_count
}

pub fn count_quantum_paths(grid: &[Vec<Cell>]) -> u128 {
    let rows = grid.len();
    let cols = grid.first().map(|r| r.len()).unwrap_or(0);

    // HashMap stores the number of distinct paths (ways) that place the beam
    // at the cell with coordinates (usize, usize).
    // so active[(r,c)] = number of distinct paths whose beam is currently at (r,c)
    let mut active: HashMap<(usize, usize), u128> = HashMap::new();
    let mut next: HashMap<(usize, usize), u128> = HashMap::new();

    // seed at S and set ways = 1
    for (r, row) in grid.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            if *cell == Cell::Source {
                active.insert((r, c), 1);
            }
        }
    }

    // Total number of complete paths that leave the grid (fall past bottom)
    let mut total_paths = 0u128;

    while !active.is_empty() {
        next.clear();

        for (&(r, c), &ways) in active.iter() {
            let r1 = r + 1;

            // if the beam would move beyond the bottom, this path terminates sucessfuly
            if r1 >= rows {
                total_paths += ways;
                continue;
            }

            match grid[r][c] {
                // each path branches into TWO possible continuations
                Cell::Splitter => {
                    if c > 0 {
                        let dest = (r1, c - 1);
                        *next.entry(dest).or_insert(0) += ways;
                    }
                    if c + 1 < cols {
                        let dest = (r1, c + 1);
                        *next.entry(dest).or_insert(0) += ways;
                    }
                }
                // Normal cell: beam goes straight down
                _ => {
                    let dest = (r1, c);
                    *next.entry(dest).or_insert(0) += ways;
                }
            }
        }

        // swap maps for next iteration (tick) of the loop
        swap(&mut active, &mut next);
    }

    total_paths
}

#[cfg(test)]
mod tests {
    use super::Cell::*;
    use super::*;

    const EXAMPLE: &str = "...S...
.......
...^...
..^.^..
.......
";
    #[test]
    fn test_parse_input() {
        let expected = vec![
            vec![Empty, Empty, Empty, Source, Empty, Empty, Empty],
            vec![Empty, Empty, Empty, Empty, Empty, Empty, Empty],
            vec![Empty, Empty, Empty, Splitter, Empty, Empty, Empty],
            vec![Empty, Empty, Splitter, Empty, Splitter, Empty, Empty],
            vec![Empty, Empty, Empty, Empty, Empty, Empty, Empty],
        ];
        let result = parse_input(EXAMPLE);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_splits() {
        let expected = 3;
        let result = count_splits(&parse_input(EXAMPLE));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_quantum_paths() {
        let expected = 4;
        let result = count_quantum_paths(&parse_input(EXAMPLE));
        assert_eq!(result, expected);
    }
}

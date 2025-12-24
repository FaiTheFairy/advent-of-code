use std::{collections::HashSet, mem::swap};

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
}

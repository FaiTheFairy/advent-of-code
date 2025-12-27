use std::collections::HashSet;

/// parses the referenced str to a grid of bytes and the starting position of the guard.
/// Note: it considers the guard position as empty ('.') as she can walk over it
pub fn parse_input(input: &str) -> (Vec<Vec<u8>>, (i32, i32)) {
    let mut grid: Vec<Vec<u8>> = vec![];
    let mut start = (0, 0);

    for (y, line) in input.lines().enumerate() {
        let mut row: Vec<u8> = vec![];
        for (x, byte) in line.as_bytes().iter().enumerate() {
            match *byte {
                b'^' => {
                    start = (x as i32, y as i32);
                    row.push(b'.');
                }
                b'#' => row.push(b'#'),
                b'.' => row.push(b'.'),
                _ => unreachable!(),
            }
        }
        grid.push(row);
    }
    (grid, start)
}

/// runs a loop simulating the guards movement, keeping track of every distinct
/// position it walks over in a HashSet. returns the length of said HashSet
pub fn count_distinct(grid: &[Vec<u8>], start: (i32, i32)) -> usize {
    // up, right, down, left
    let dirs = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut dir_idx = 0; // starts facing up
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let mut pos = start;
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    loop {
        visited.insert(pos);

        let (dx, dy) = dirs[dir_idx];
        let next = (pos.0 + dx, pos.1 + dy);

        // left the map
        if next.0 < 0 || next.1 < 0 || next.0 >= width || next.1 >= height {
            break;
        }

        // turn right if obstacle ahead
        // grid[y][x]
        if grid[next.1 as usize][next.0 as usize] == b'#' {
            dir_idx = (dir_idx + 1) % 4;
        } else {
            // move forward
            pos = next;
        }
    }
    visited.len()
}

pub fn loops_with_obstruction(grid: &[Vec<u8>], start: (i32, i32), block: (i32, i32)) -> bool {
    let dirs = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut dir_idx = 0usize;

    let mut pos = start;
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    let mut seen: HashSet<(i32, i32, usize)> = HashSet::new();

    loop {
        if !seen.insert((pos.0, pos.1, dir_idx)) {
            return true; // loop
        }

        let (dx, dy) = dirs[dir_idx];
        let next = (pos.0 + dx, pos.1 + dy);

        if next.0 < 0 || next.1 < 0 || next.0 >= width || next.1 >= height {
            return false; // exited map
        }

        let blocked_by_wall = grid[next.1 as usize][next.0 as usize] == b'#';
        let blocked_by_new = next == block;

        if blocked_by_wall || blocked_by_new {
            dir_idx = (dir_idx + 1) % 4;
        } else {
            pos = next;
        }
    }
}

pub fn count_loop_positions(grid: &[Vec<u8>], start: (i32, i32)) -> usize {
    let height = grid.len();
    let width = grid[0].len();
    let mut count = 0;

    for y in 0..height {
        for x in 0..width {
            let p = (x as i32, y as i32);

            if p == start {
                continue;
            }
            if grid[y][x] == b'#' {
                continue;
            }
            if loops_with_obstruction(grid, start, p) {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn test_parse_input() {
        let result = parse_input(EXAMPLE);
        let expected = (
            vec![
                vec![b'.', b'.', b'.', b'.', b'#', b'.', b'.', b'.', b'.', b'.'],
                vec![b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'#'],
                vec![b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.'],
                vec![b'.', b'.', b'#', b'.', b'.', b'.', b'.', b'.', b'.', b'.'],
                vec![b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'#', b'.', b'.'],
                vec![b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.'],
                vec![b'.', b'#', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.'],
                vec![b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'#', b'.'],
                vec![b'#', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.'],
                vec![b'.', b'.', b'.', b'.', b'.', b'.', b'#', b'.', b'.', b'.'],
            ],
            (4, 6),
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn test_count_distinct() {
        let (grid, start) = parse_input(EXAMPLE);
        let result = count_distinct(&grid, start);
        let expected = 41usize;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_loop_positions() {
        let (grid, start) = parse_input(EXAMPLE);
        let result = count_loop_positions(&grid, start);
        let expected = 6usize;
        assert_eq!(result, expected);
    }
}

pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut grid = vec![];
    for line in input.lines() {
        let mut row: Vec<char> = vec![];
        for char in line.chars() {
            row.push(char);
        }
        grid.push(row);
    }
    grid
}

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

pub fn count_xmas(grid: &[Vec<char>]) -> u64 {
    let mut count = 0u64;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'X' {
                for (dx, dy) in DIRECTIONS {
                    if matches_xmas(grid, x, y, dx, dy) {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

pub fn count_x_mas(grid: &[Vec<char>]) -> u64 {
    let h = grid.len();
    let w = grid[0].len();
    let mut count: u64 = 0;

    // center can't be on border
    for y in 1..(h - 1) {
        for x in 1..(w - 1) {
            if grid[y][x] != 'A' {
                continue;
            }

            let tl = grid[y - 1][x - 1];
            let tr = grid[y - 1][x + 1];
            let bl = grid[y + 1][x - 1];
            let br = grid[y + 1][x + 1];

            let diag1 = (tl == 'M' && br == 'S') || (tl == 'S' && br == 'M');
            let diag2 = (tr == 'M' && bl == 'S') || (tr == 'S' && bl == 'M');

            if diag1 && diag2 {
                count += 1;
            }
        }
    }

    count
}

fn matches_xmas(grid: &[Vec<char>], x: usize, y: usize, dx: i32, dy: i32) -> bool {
    let h = grid.len() as i32;
    let w = grid[0].len() as i32;

    let word = ['X', 'M', 'A', 'S'];

    for i in 0..4 {
        let nx = x as i32 + dx * i;
        let ny = y as i32 + dy * i;

        if nx < 0 || ny < 0 || nx >= w || ny >= h {
            return false;
        }

        if grid[ny as usize][nx as usize] != word[i as usize] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_parse_input() {
        let result = parse_input(EXAMPLE);
        let expected = vec![
            vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
            vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
            vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
            vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
            vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
            vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
            vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
            vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
            vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
            vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_xmas() {
        let grid = parse_input(EXAMPLE);
        let result = count_xmas(&grid);
        assert_eq!(result, 18);
    }

    #[test]
    fn test_count_x_mas() {
        let grid = parse_input(EXAMPLE);
        let result = count_x_mas(&grid);
        assert_eq!(result, 9);
    }
}

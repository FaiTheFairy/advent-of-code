use std::{collections::HashSet, fs, str::FromStr};

use anyhow::{Result, anyhow, bail, ensure};

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let sol1 = solve_part_1(&input)?;
    println!("Part 1. after one fold instruction, {sol1} dots are visible.");

    let sol2 = solve_part_2(&input)?;
    println!("Part 2.\n\n{sol2}");
    Ok(())
}

fn solve_part_1(input: &str) -> Result<usize> {
    let mut input = input.parse::<Input>()?;
    input.apply_folds_up_to(1)?;
    Ok(input.paper.visible_dots())
}

fn solve_part_2(input: &str) -> Result<String> {
    let mut input = input.parse::<Input>()?;
    input.apply_all_folds();
    Ok(input.paper.render())
}

type Coord = (usize, usize);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Input {
    paper: Paper,
    folds: Vec<Fold>,
}

impl Input {
    fn apply_all_folds(&mut self) {
        for fold in &self.folds {
            self.paper.apply_fold(fold);
        }
    }

    fn apply_folds_up_to(&mut self, num_folds: usize) -> Result<()> {
        ensure!(
            num_folds <= self.folds.len(),
            "number of folds requested exceeds fold instructions"
        );

        for i in 0..num_folds {
            self.paper.apply_fold(&self.folds[i]);
        }

        Ok(())
    }
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (dots, instructions) = s.split_once("\n\n").ok_or(anyhow!(
            "empty line between dots and instructions not found."
        ))?;

        let paper = dots.parse::<Paper>()?;
        // let mut folds = Vec::with_capacity(instructions.lines().count());
        let folds = instructions
            .lines()
            .map(str::parse::<Fold>)
            .collect::<Result<_, _>>()?;

        Ok(Self { paper, folds })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Paper {
    dots: HashSet<Coord>,
    width: usize,
    height: usize,
}

impl Paper {
    fn apply_fold(&mut self, fold: &Fold) {
        let mut new_dots = HashSet::with_capacity(self.visible_dots());

        for &(x, y) in &self.dots {
            let (x, y) = match fold {
                Fold::AlongX(k) if x > *k => (2 * k - x, y),
                Fold::AlongY(k) if y > *k => (x, 2 * k - y),
                _ => (x, y),
            };

            new_dots.insert((x, y));
        }

        match fold {
            Fold::AlongX(_) => self.width /= 2,
            Fold::AlongY(_) => self.height /= 2,
        }

        self.dots = new_dots;
    }

    fn visible_dots(&self) -> usize {
        self.dots.len()
    }

    fn render(&self) -> String {
        let w = self.width;
        let h = self.height;

        let mut out = String::with_capacity(w * h);

        for y in 0..h {
            for x in 0..w {
                if self.dots.contains(&(x, y)) {
                    out.push('#');
                } else {
                    out.push('.');
                }
            }
            // add new line except for the last line
            if y + 1 != h {
                out.push('\n');
            }
        }
        out
    }
}

impl FromStr for Paper {
    type Err = anyhow::Error;

    /// Parses
    /// ```text
    /// 6,10
    /// 0,14
    /// 9,10
    /// ```
    /// to `Paper`
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut dots = HashSet::with_capacity(s.lines().count());
        for line in s.lines() {
            let line = line.trim();
            let (row, col) = line
                .split_once(",")
                .ok_or_else(|| anyhow!("malformed dot coordinates: {line}"))?;
            let row = row.parse::<usize>()?;
            let col = col.parse::<usize>()?;
            dots.insert((row, col));
        }
        let width = *dots.iter().map(|(x, _)| x).max().unwrap() + 1;
        let height = *dots.iter().map(|(_, y)| y).max().unwrap() + 1;
        Ok(Self {
            dots,
            width,
            height,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Fold {
    AlongX(usize),
    AlongY(usize),
}

impl FromStr for Fold {
    type Err = anyhow::Error;

    /// Parses "fold along y=7" to `Fold::AlongY(7)`
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.trim().strip_prefix("fold along ") {
            Some(stripped) => {
                let (dir, num) = stripped
                    .split_once('=')
                    .ok_or_else(|| anyhow!("malformed fold instruction: {stripped}"))?;

                let num = num.parse::<usize>()?;
                match dir {
                    "x" => Ok(Self::AlongX(num)),
                    "y" => Ok(Self::AlongY(num)),
                    _ => bail!("unknown direction: {dir}"),
                }
            }
            None => bail!("malformed fold instruction: {s}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn test_solve_part_1() {
        let result = solve_part_1(EXAMPLE).unwrap();
        assert_eq!(result, 17);
    }

    #[test]
    fn test_apply_all_folds() {
        let mut input = EXAMPLE.parse::<Input>().unwrap();
        input.apply_all_folds();
        let result = input.paper.render();
        let expected = "#####
#...#
#...#
#...#
#####
.....
.....";

        assert_eq!(result, expected);
    }

    #[test]
    fn test_apply_fold_once() {
        let mut input = EXAMPLE.parse::<Input>().unwrap();
        input.apply_folds_up_to(1).unwrap();
        let result = input.paper.render();
        let expected = "#.##..#..#.
#...#......
......#...#
#...#......
.#.#..#.###
...........
...........";

        assert_eq!(result, expected);
    }

    #[test]
    fn test_render_paper() {
        let result = EXAMPLE.parse::<Input>().unwrap().paper.render();
        let expected = "...#..#..#.
....#......
...........
#..........
...#....#.#
...........
...........
...........
...........
...........
.#....#.##.
....#......
......#...#
#..........
#.#........";

        assert_eq!(result, expected);
    }

    #[test]
    fn test_paper_dimensions() {
        let paper = EXAMPLE.parse::<Input>().unwrap().paper;
        assert_eq!(paper.width, 11);
        assert_eq!(paper.height, 15);
    }

    #[test]
    fn test_parse_input() {
        let result = "6,10
0,14
9,10

fold along y=6
fold along x=2"
            .parse::<Input>()
            .unwrap();

        let expected = Input {
            paper: Paper {
                dots: HashSet::from([(6, 10), (0, 14), (9, 10)]),
                width: 10,
                height: 15,
            },
            folds: vec![Fold::AlongY(6), Fold::AlongX(2)],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_paper() {
        let result = "6,10\n0,14\n9,10".parse::<Paper>().unwrap();
        let expected = Paper {
            dots: HashSet::from([(6, 10), (0, 14), (9, 10)]),
            width: 10,
            height: 15,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_fold() {
        let result_y7 = "fold along y=7".parse::<Fold>().unwrap();
        assert_eq!(result_y7, Fold::AlongY(7));

        let result_x2 = "fold along x=2".parse::<Fold>().unwrap();
        assert_eq!(result_x2, Fold::AlongX(2));

        assert!("xx=2".parse::<Fold>().is_err());
    }
}

use std::fs;
use std::str::FromStr;

use anyhow::Context;
use anyhow::Result;
use anyhow::bail;
use anyhow::ensure;

const TOTAL_DISK_SPACE: u64 = 70_000_000;
const UPDATE_SIZE: u64 = 30_000_000;

fn main() -> Result<()> {
    let input = fs::read_to_string("./input.txt")?;
    let problem_input = input.parse::<ProblemInput>()?;

    let sol1 = problem_input.solve_part_1()?;
    println!("Part 1. solution = {sol1}");

    let sol2 = problem_input.solve_part_2()?;
    println!("Part 2. solution = {sol2}");

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Command {
    CdRoot,
    CdUp,
    CdDown(String),
    Ls,
}

impl FromStr for Command {
    type Err = anyhow::Error;

    /// Parses
    /// ```
    /// "$ cd /"
    /// ```
    /// to `Command`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .strip_prefix("$ ")
            .context("Command doesn't start with '$ '")?;
        match s {
            "ls" => Ok(Self::Ls),
            "cd /" => Ok(Self::CdRoot),
            "cd .." => Ok(Self::CdUp),
            s if let Some(arg) = s.strip_prefix("cd ") => Ok(Self::CdDown(arg.trim().into())),
            _ => bail!("Unknown command: {s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Listing {
    Dir(String),
    File { size: u64, name: String },
}

impl FromStr for Listing {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s1, s2) = s
            .split_once(" ")
            .with_context(|| format!("malformed entry: {s}"))?;
        if s1 == "dir" {
            Ok(Listing::Dir(s2.into()))
        } else {
            Ok(Listing::File {
                size: s1.parse::<u64>().context("parsing file size")?,
                name: s2.into(),
            })
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Line {
    Command(Command),
    Listing(Listing),
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.starts_with("$") {
            true => Ok(Self::Command(s.parse::<Command>()?)),
            false => Ok(Self::Listing(s.parse::<Listing>()?)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ProblemInput(Vec<Line>);

impl FromStr for ProblemInput {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(str::parse::<Line>)
                .collect::<Result<Vec<_>>>()?,
        ))
    }
}

impl ProblemInput {
    fn ingest(&self) -> Result<Analyzer> {
        let mut analyzer = Analyzer::default();
        for line in self.0.iter() {
            analyzer.ingest(line)?;
        }
        analyzer.finish()?;
        Ok(analyzer)
    }

    fn solve_part_1(&self) -> Result<u64> {
        let analyzer = self.ingest()?;
        Ok(analyzer.sum_dirs_at_most(100_000))
    }

    fn solve_part_2(&self) -> Result<u64> {
        let analyzer = self.ingest()?;
        let root_total = analyzer.root_total()?;
        let unused_space = TOTAL_DISK_SPACE - root_total;
        let space_needed = UPDATE_SIZE.saturating_sub(unused_space);

        analyzer
            .smallest_dir_at_least(space_needed)
            .context("no directory has enough size")
    }
}

#[derive(Debug, Default, Clone)]
struct Analyzer {
    stack: Vec<u64>,
    totals: Vec<u64>,
}

impl Analyzer {
    fn ingest(&mut self, line: &Line) -> Result<()> {
        match line {
            Line::Command(cmd) => match cmd {
                Command::CdRoot => {
                    self.stack.clear();
                    self.stack.push(0);
                }
                Command::CdUp => {
                    self.close_current_dir()?;
                }
                Command::CdDown(_) => {
                    if self.stack.is_empty() {
                        bail!("cd into directory before cd /");
                    }
                    self.stack.push(0);
                }
                Command::Ls => (),
            },

            Line::Listing(listing) => match listing {
                Listing::Dir(_) => (),
                Listing::File { size, name: _ } => {
                    let cur = self
                        .stack
                        .last_mut()
                        .context("file listing encountered before cd /")?;
                    *cur += *size;
                }
            },
        }

        Ok(())
    }

    fn finish(&mut self) -> Result<()> {
        while self.stack.len() > 1 {
            self.close_current_dir()?;
        }

        // record root if present
        if let Some(&root) = self.stack.last() {
            self.totals.push(root);
        }

        Ok(())
    }

    fn close_current_dir(&mut self) -> Result<()> {
        ensure!(self.stack.len() >= 2, "cd .. at root (no parent directory)");

        let child_total = self
            .stack
            .pop()
            .context("internal error: stack underflow")?;

        // record directory total
        self.totals.push(child_total);

        let parent = self
            .stack
            .last_mut()
            .context("internal error: missing parent after pop")?;
        *parent += child_total;

        Ok(())
    }

    fn sum_dirs_at_most(&self, limit: u64) -> u64 {
        self.totals.iter().copied().filter(|&t| t <= limit).sum()
    }

    fn smallest_dir_at_least(&self, needed: u64) -> Option<u64> {
        self.totals.iter().copied().filter(|&t| t >= needed).min()
    }

    fn root_total(&self) -> Result<u64> {
        self.totals
            .last()
            .copied()
            .context("no totals recorded; did you call `finish()`?")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_parse_line_command_ls() -> Result<()> {
        let input = "$ ls";
        let result = input.parse::<Line>()?;
        let expected = Line::Command(Command::Ls);
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_parse_line_command_cd_root() -> Result<()> {
        let input = "$ cd /";
        let result = input.parse::<Line>()?;
        let expected = Line::Command(Command::CdRoot);
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_parse_line_command_cd_dir() -> Result<()> {
        let input = "$ cd a";
        let result = input.parse::<Line>()?;
        let expected = Line::Command(Command::CdDown("a".into()));
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_parse_line_command_cd_up() -> Result<()> {
        let input = "$ cd ..";
        let result = input.parse::<Line>()?;
        let expected = Line::Command(Command::CdUp);
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_parse_problem_input() -> Result<()> {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d";
        let result = input.parse::<ProblemInput>()?;
        let expected = ProblemInput(vec![
            Line::Command(Command::CdRoot),
            Line::Command(Command::Ls),
            Line::Listing(Listing::Dir("a".into())),
            Line::Listing(Listing::File {
                size: 14_848_514,
                name: "b.txt".into(),
            }),
            Line::Listing(Listing::File {
                size: 8_504_156,
                name: "c.dat".into(),
            }),
            Line::Listing(Listing::Dir("d".into())),
        ]);
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_solve_part_1() -> Result<()> {
        let result = EXAMPLE
            .parse::<ProblemInput>()?
            .solve_part_1()
            .context("Couldn't solve part 1")?;
        assert_eq!(result, 95437);
        Ok(())
    }

    #[test]
    fn test_root_total() -> Result<()> {
        let result = EXAMPLE.parse::<ProblemInput>()?.ingest()?.root_total()?;
        assert_eq!(result, 48381165);
        Ok(())
    }

    #[test]
    fn test_solve_part_2() -> Result<()> {
        let result = EXAMPLE.parse::<ProblemInput>()?.solve_part_2()?;
        assert_eq!(result, 24933642);
        Ok(())
    }
}

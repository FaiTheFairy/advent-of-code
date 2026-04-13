use std::{fs, str::FromStr};

use anyhow::{Result, bail};

fn main() -> Result<()> {
    let instructions = fs::read_to_string("input.txt")?.parse()?;
    let start = Position { row: 1, col: 1 };
    let sol1 = PART_1_KEYPAD.code(start, &instructions)?;
    println!("Part 1: {sol1}");

    let sol2 = PART_2_KEYPAD.code(start, &instructions)?;
    println!("Part 2: {sol2}");

    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct InstructionLine(Vec<Move>);

impl InstructionLine {
    fn iter(&self) -> impl Iterator<Item = Move> + '_ {
        self.0.iter().copied()
    }
}

impl FromStr for InstructionLine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let moves: Vec<Move> = s.chars().map(Move::try_from).collect::<Result<_, _>>()?;
        Ok(Self(moves))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Instructions(Vec<InstructionLine>);

impl Instructions {
    fn iter(&self) -> impl Iterator<Item = &InstructionLine> {
        self.0.iter()
    }
}

impl FromStr for Instructions {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<InstructionLine> = s
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        Ok(Self(lines))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Position {
    row: isize,
    col: isize,
}

impl Position {
    fn moved(self, mv: Move) -> Self {
        match mv {
            Move::Up => Self {
                row: self.row - 1,
                col: self.col,
            },
            Move::Down => Self {
                row: self.row + 1,
                col: self.col,
            },
            Move::Left => Self {
                row: self.row,
                col: self.col - 1,
            },
            Move::Right => Self {
                row: self.row,
                col: self.col + 1,
            },
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Move {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'U' => Ok(Self::Up),
            'D' => Ok(Self::Down),
            'R' => Ok(Self::Right),
            'L' => Ok(Self::Left),
            _ => bail!("unknown move direction: {value}"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Keypad<const H: usize, const W: usize> {
    buttons: [[Option<char>; W]; H],
}

impl<const H: usize, const W: usize> Keypad<H, W> {
    fn button_at(&self, pos: Position) -> Option<char> {
        let row: usize = usize::try_from(pos.row).ok()?;
        let col: usize = usize::try_from(pos.col).ok()?;
        self.buttons.get(row)?.get(col).copied().flatten()
    }

    fn step(&self, pos: Position, mv: Move) -> Position {
        let next: Position = pos.moved(mv);
        if self.button_at(next).is_some() {
            next
        } else {
            pos
        }
    }

    fn follow_line(&self, start: Position, line: &InstructionLine) -> Position {
        line.iter().fold(start, |pos, mv| self.step(pos, mv))
    }

    fn code(&self, start: Position, instructions: &Instructions) -> Result<String> {
        let mut pos: Position = start;
        let mut code: String = String::new();

        for line in instructions.iter() {
            pos = self.follow_line(pos, line);
            let button: char = self
                .button_at(pos)
                .ok_or_else(|| anyhow::anyhow!("invalid final position"))?;
            code.push(button);
        }

        Ok(code)
    }
}

const PART_1_KEYPAD: Keypad<3, 3> = Keypad {
    buttons: [
        [Some('1'), Some('2'), Some('3')],
        [Some('4'), Some('5'), Some('6')],
        [Some('7'), Some('8'), Some('9')],
    ],
};

#[rustfmt::skip]
const PART_2_KEYPAD: Keypad<5, 5> = Keypad {
    buttons: [
        [None     , None     , Some('1'), None     , None]     ,
        [None     , Some('2'), Some('3'), Some('4'), None]     ,
        [Some('5'), Some('6'), Some('7'), Some('8'), Some('9')],
        [None     , Some('A'), Some('B'), Some('C'), None]     ,
        [None     , None     , Some('D'), None     , None]
        
    ]
};

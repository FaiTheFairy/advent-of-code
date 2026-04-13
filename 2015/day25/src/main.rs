use anyhow::Result;

fn main() -> Result<()> {
    let sol1 = code_at(Position {
        row: 2947,
        column: 3029,
    });
    println!("Part 1: {}", sol1.0);

    Ok(())
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Code(u64);

impl Code {
    const INITIAL: Self = Self(20_151_125);
    const MULTIPLIER: u64 = 252_533;
    const MODULUS: u64 = 33_554_393;

    fn next(self) -> Code {
        Code(self.0 * Self::MULTIPLIER % Self::MODULUS)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Position {
    row: usize,
    column: usize,
}

impl Position {
    fn sequence_index(self) -> usize {
        triangular(self.row + self.column - 2) + self.column
    }
}

fn triangular(n: usize) -> usize {
    n * (n + 1) / 2
}

fn code_at(position: Position) -> Code {
    let target = position.sequence_index();
    let mut code = Code::INITIAL;

    for _ in 1..target {
        code = code.next();
    }

    code
}

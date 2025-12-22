use crate::dial::TurnDial;
use crate::parse::parse_turn;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn extract_input_file(filename: &str) -> io::Result<Vec<TurnDial>> {
    let lines = read_lines(filename)?;
    lines.map(|line| line.map(parse_turn)).collect()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

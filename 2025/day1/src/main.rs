mod dial;
mod parse;
mod read_file;

use std::path::PathBuf;

use dial::Dial;
use read_file::extract_input_file;

fn main() {
    // generates a dial at position 50 and 0 counts of stopping at zero
    let mut dial = Dial::new();
    let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("input.txt");

    let dial_turns = match extract_input_file(filename) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error reading input: {e}");
            return;
        }
    };

    for turn in dial_turns {
        dial.turn(turn);
    }
    println!("{:?}", dial);
}

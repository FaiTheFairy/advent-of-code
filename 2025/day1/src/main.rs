mod dial;
mod parse;
mod read_file;

use dial::Dial;
use read_file::extract_input_file;

fn main() {
    // generates a dial at position 50 and 0 counts of stopping at zero
    let mut dial = Dial::new();

    let dial_turns = match extract_input_file("input.txt") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error reading input: {e}");
            return;
        }
    };

    for turn in dial_turns {
        dial.turn(turn);
    }
    // let turn1 = TurnDial {
    //     direction: Direction::Right,
    //     increment: 10,
    // };

    // let turn2 = TurnDial {
    //     direction: Direction::Left,
    //     increment: 40,
    // };

    // dial.turn(turn1);
    // dial.turn(turn2);

    println!("{:?}", dial);
}

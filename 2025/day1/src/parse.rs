use crate::dial::{Direction, TurnDial};

pub fn parse_turn(input: String) -> TurnDial {
    let (letter, number) = input.split_at(1);
    let direction = match letter {
        "R" => Direction::Right,
        "L" => Direction::Left,
        _ => panic!("Direction of turn not valid (has to be either 'L' or 'R')"),
    };

    let increment: u32 = number.parse().unwrap();

    TurnDial {
        direction,
        increment,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_left_turn_works() {
        let input = String::from("R18");
        let result = parse_turn(input);
        let expected = TurnDial {
            direction: Direction::Right,
            increment: 18,
        };
        assert_eq!(result, expected);
    }
}

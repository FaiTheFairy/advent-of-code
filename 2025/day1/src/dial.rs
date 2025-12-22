#[derive(Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
pub struct TurnDial {
    pub direction: Direction,
    pub increment: u32,
}

#[derive(Debug)]
pub struct Dial {
    position: u32,
    count_land_on_zero: u32,
}

impl Dial {
    pub fn new() -> Self {
        Self {
            position: 50,
            count_land_on_zero: 0,
        }
    }
    pub fn turn(&mut self, turn: TurnDial) {
        match turn.direction {
            Direction::Right => self.turn_right(turn.increment),
            Direction::Left => self.turn_left(turn.increment),
        };
    }

    fn turn_right(&mut self, increment: u32) {
        let dist = if self.position == 0 {
            100
        } else {
            100 - self.position
        };
        let hits = if increment < dist {
            0
        } else {
            1 + (increment - dist) / 100
        };
        self.position = (self.position + increment) % 100;
        self.count_land_on_zero += hits;
    }

    fn turn_left(&mut self, increment: u32) {
        let dist = if self.position == 0 {
            100
        } else {
            self.position
        };
        let hits = if increment < dist {
            0
        } else {
            1 + (increment - dist) / 100
        };
        self.position = (self.position + 100 - (increment % 100)) % 100;
        self.count_land_on_zero += hits;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn l(n: u32) -> TurnDial {
        TurnDial {
            direction: Direction::Left,
            increment: n,
        }
    }

    fn r(n: u32) -> TurnDial {
        TurnDial {
            direction: Direction::Right,
            increment: n,
        }
    }

    #[test]
    fn part2_example_sequence_positions_and_hits() {
        // starting pointing at 50 with zero times pointing to 0
        let mut dial = Dial::new();

        // L68 -> 82, hits 0 once during rotation
        dial.turn(l(68));
        assert_eq!(dial.position, 82);
        assert_eq!(dial.count_land_on_zero, 1);

        // L30 -> 52, no extra hits
        dial.turn(l(30));
        assert_eq!(dial.position, 52);
        assert_eq!(dial.count_land_on_zero, 1);

        // R48 -> 0, counts (lands on 0)
        dial.turn(r(48));
        assert_eq!(dial.position, 0);
        assert_eq!(dial.count_land_on_zero, 2);

        // L5 -> 95, no extra hits
        dial.turn(l(5));
        assert_eq!(dial.position, 95);
        assert_eq!(dial.count_land_on_zero, 2);

        // R60 -> 55, hits 0 once during rotation
        dial.turn(r(60));
        assert_eq!(dial.position, 55);
        assert_eq!(dial.count_land_on_zero, 3);

        // L55 -> 0, counts (lands on 0)
        dial.turn(l(55));
        assert_eq!(dial.position, 0);
        assert_eq!(dial.count_land_on_zero, 4);

        // L1 -> 99, no extra hits
        dial.turn(l(1));
        assert_eq!(dial.position, 99);
        assert_eq!(dial.count_land_on_zero, 4);

        // L99 -> 0, counts (lands on 0)
        dial.turn(l(99));
        assert_eq!(dial.position, 0);
        assert_eq!(dial.count_land_on_zero, 5);

        // R14 -> 14, no extra hits
        dial.turn(r(14));
        assert_eq!(dial.position, 14);
        assert_eq!(dial.count_land_on_zero, 5);

        // L82 -> 32, hits 0 once during rotation
        dial.turn(l(82));
        assert_eq!(dial.position, 32);
        assert_eq!(dial.count_land_on_zero, 6);
    }

    #[test]
    fn part2_r1000_from_50_hits_ten_and_returns_to_50() {
        let mut dial = Dial {
            position: 50,
            count_land_on_zero: 0,
        };

        dial.turn(r(1000));

        assert_eq!(dial.position, 50);
        assert_eq!(dial.count_land_on_zero, 10);
    }
}

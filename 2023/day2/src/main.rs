fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let sum1 = sum_part_1(&input);
    println!("Part 1. Sum of valid game IDs = {sum1}");

    let sum2 = sum_part_2(&input);
    println!("Part 2. Sum of of power of min sets = {sum2}");
}

const RED_CUBES: u32 = 12;
const GREEN_CUBES: u32 = 13;
const BLUE_CUBES: u32 = 14;

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

#[derive(Debug, PartialEq, Eq)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

impl Set {
    fn parse(set: &str) -> Set {
        let cubes = set.split(',');
        let (mut r, mut g, mut b) = (0, 0, 0);
        for cube in cubes {
            let (num, color) = cube
                .trim()
                .split_once(" ")
                .expect("cube not in proper formatting");
            let num = num.parse::<u32>().expect("Couldn't parse number");
            match color {
                "red" => r = num,
                "green" => g = num,
                "blue" => b = num,
                _ => panic!("Unknown color: {color}"),
            }
        }

        Set {
            red: r,
            green: g,
            blue: b,
        }
    }
    fn is_valid(&self) -> bool {
        self.red <= RED_CUBES && self.green <= GREEN_CUBES && self.blue <= BLUE_CUBES
    }
}

impl Game {
    fn parse(game: &str) -> Game {
        let (game_id, rest) = game.split_once(':').expect("No colon found for game.");
        let id = game_id.strip_prefix("Game ").unwrap().parse().unwrap();

        let sets: Vec<Set> = rest.split(';').map(Set::parse).collect();

        Game { id, sets }
    }

    fn is_valid(&self) -> bool {
        self.sets.iter().all(Set::is_valid)
    }

    fn min_set_of_cubes(&self) -> Set {
        Set {
            red: self.sets.iter().map(|s| s.red).max().unwrap_or(0),
            green: self.sets.iter().map(|s| s.green).max().unwrap_or(0),
            blue: self.sets.iter().map(|s| s.blue).max().unwrap_or(0),
        }
    }
}

fn sum_part_1(input: &str) -> u32 {
    input
        .lines()
        .map(Game::parse)
        .filter(Game::is_valid)
        .map(|game| game.id)
        .sum()
}

fn sum_part_2(input: &str) -> u32 {
    input
        .lines()
        .map(Game::parse)
        .map(|game| game.min_set_of_cubes())
        .map(|set| set.red * set.green * set.blue)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_set() {
        let set = "3 blue";
        assert_eq!(
            Set::parse(set),
            Set {
                red: 0,
                green: 0,
                blue: 3,
            }
        );

        let set = "1 red, 2 green, 6 blue";
        assert_eq!(
            Set::parse(set),
            Set {
                red: 1,
                green: 2,
                blue: 6
            }
        );
    }

    #[test]
    fn test_parse_game() {
        let game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        assert_eq!(
            Game::parse(game),
            Game {
                id: 1,
                sets: vec![
                    Set {
                        red: 4,
                        green: 0,
                        blue: 3
                    },
                    Set {
                        red: 1,
                        green: 2,
                        blue: 6
                    },
                    Set {
                        red: 0,
                        green: 2,
                        blue: 0
                    }
                ]
            }
        )
    }

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part_1() {
        assert_eq!(sum_part_1(INPUT), 8u32);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(sum_part_2(INPUT), 2286u32)
    }
}

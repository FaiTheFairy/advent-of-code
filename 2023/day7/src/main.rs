use std::{collections::HashMap, fs};

use anyhow::{Context, Result, bail};

fn main() -> Result<()> {
    let input = fs::read_to_string("./input.txt")?;
    let sum = solve_part_1(&input)?;

    println!("Part 1. Sum = {sum}");

    let sum2 = solve_part_2(&input)?;
    println!("Part 2. Sum = {sum2}");

    Ok(())
}

fn solve_part_1(input: &str) -> Result<u64> {
    let parsed = parse_input(input, false)?;
    let bids_and_ranks = get_bids_and_ranks(&parsed);
    let sum = get_total_winnings(&bids_and_ranks);
    Ok(sum)
}

fn solve_part_2(input: &str) -> Result<u64> {
    let parsed = parse_input(input, true)?;
    let bids_and_ranks = get_bids_and_ranks(&parsed);
    let sum = get_total_winnings(&bids_and_ranks);
    Ok(sum)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn try_from(c: char, with_joker: bool) -> Result<Self> {
        match c {
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            'T' => Ok(Self::Ten),
            'J' => match with_joker {
                true => Ok(Self::Joker),
                false => Ok(Self::Jack),
            },
            'Q' => Ok(Self::Queen),
            'K' => Ok(Self::King),
            'A' => Ok(Self::Ace),
            _ => bail!("Couldn't match card value to enum"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Strength {
    // 5 unique
    HighCard,
    // 4 unique
    OnePair,
    // 3 unique
    TwoPair,
    // 3 unique
    ThreeOfKind,
    // 2 unique
    FullHouse,
    // 2 unique
    FourOfKind,
    // 1 card
    FiveOfKind,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Hand {
    strength: Strength,
    cards: Vec<Card>,
    bid: u64,
    with_joker: bool,
}

// impl PartialOrd for Hand {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         match self.strength.partial_cmp(&other.strength) {
//             Some(core::cmp::Ordering::Equal) => {}
//             ord => return ord,
//         }
//         match self.cards.partial_cmp(&other.cards) {
//             Some(core::cmp::Ordering::Equal) => {}
//             ord => return ord,
//         }
//         self.bid.partial_cmp(&other.bid)
//     }
// }

fn parse_input(input: &str, with_joker: bool) -> Result<Vec<Hand>> {
    let mut out = Vec::new();
    for line in input.lines() {
        let (hand, bid) = line
            .split_once(" ")
            .with_context(|| format!("Couldn't split line {line} at whitespace"))?;

        let cards: Vec<Card> = hand
            .chars()
            .map(|c| Card::try_from(c, with_joker).unwrap())
            .collect();
        let strength: Strength = strength(&cards, with_joker)?;
        let bid = bid.parse::<u64>()?;
        out.push(Hand {
            cards,
            strength,
            bid,
            with_joker,
        });
    }

    Ok(out)
}

/*
We will utilize a HashMap<Card, count> and match that to the Strength
*/
fn strength(cards: &[Card], with_joker: bool) -> Result<Strength> {
    use Strength::*;

    let mut card_count: HashMap<&Card, usize> = HashMap::new();

    for card in cards.iter() {
        card_count
            .entry(card)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut cards_modified;
    if with_joker {
        cards_modified = cards.to_vec();
        // Note: if all cards are jokers returns `None`.
        let card_repeated_most = card_count
            .iter()
            // we need to exclude jokers so that the joker isn't the most repeated card.
            // if we didn't, we may simply replace jokers with jokers.
            // e.g.
            // (J, 2), (K, 2)
            // Desired behavior -> (K, 4)
            // Unwanted, but possible, behavior -> (J, 4)
            .filter(|&(&c, _)| c != &Card::Joker)
            .max_by(|&a, &b| a.1.cmp(b.1))
            .map(|(&k, _v)| k);

        for card in cards_modified.iter_mut() {
            if let Some(most) = card_repeated_most
                && card == &Card::Joker
            {
                *card = *most
            }
        }
        card_count = HashMap::new();

        for card in cards_modified.iter() {
            card_count
                .entry(card)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }

    // for 2 unique cards:     {(K, 2), (Q, 3)}       OR     {(K, 4), (Q, 1)}
    // for 3 unique cards: {(K, 2), (Q, 2), (A, 1)}   OR   {(K,3), (Q, 1), (A, 1)}
    match card_count.len() {
        1 => Ok(FiveOfKind),
        2 => {
            if card_count.values().any(|v| *v == 4) {
                Ok(FourOfKind)
            } else {
                Ok(FullHouse)
            }
        }
        3 => {
            if card_count.values().any(|v| *v == 3) {
                Ok(ThreeOfKind)
            } else {
                Ok(TwoPair)
            }
        }
        4 => Ok(OnePair),
        5 => Ok(HighCard),
        _ => bail!(
            "More than 5 unique cards found within hand of 5 cards. You cheat. (Or all your cards were stolen)"
        ),
    }
}

// fn strength_old(cards: &[Card]) -> Result<Strength> {
//     use Strength::*;

//     let unique: HashSet<Card> = cards.iter().copied().collect();

//     match unique.len() {
//         1 => Ok(FiveOfKind),
//         // full house or four of a kind
//         2 => {
//             let count = count_max_repitions(cards, &unique);
//             if count == 4 {
//                 Ok(FourOfKind)
//             } else {
//                 Ok(FullHouse)
//             }
//         }
//         // Three of a kind or two pair
//         3 => {
//             let count = count_max_repitions(cards, &unique);
//             if count == 3 {
//                 Ok(ThreeOfKind)
//             } else {
//                 Ok(TwoPair)
//             }
//         }
//         4 => Ok(OnePair),
//         5 => Ok(HighCard),
//         _ => bail!("HASHSET LEN HAS TO BE WITHIN 1 TO 5"),
//     }
// }

// fn count_max_repitions(cards: &[Card], unique: &HashSet<Card>) -> usize {
//     let mut count = 0;
//     for i in 0..unique.len() {
//         count = count.max(
//             cards
//                 .iter()
//                 .filter(|&c| c == unique.iter().nth(i).unwrap())
//                 .count(),
//         )
//     }
//     count
// }

fn get_bids_and_ranks(parsed: &[Hand]) -> Vec<(u64, u64)> {
    let mut parsed: Vec<Hand> = parsed.to_vec();
    parsed.sort_unstable();
    let mut out = Vec::with_capacity(parsed.len());
    for (idx, hand) in parsed.iter().enumerate() {
        out.push((hand.bid, idx as u64 + 1));
    }
    out
}

// Takes a slice of (bid, rank), computes the product for every hand,
// then sums up all the winnings
fn get_total_winnings(bids_and_ranks: &[(u64, u64)]) -> u64 {
    bids_and_ranks.iter().map(|(bid, rank)| bid * rank).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use Card::*;
    use Strength::*;

    const EXAMPLE_SIMPLE: &str = "32T3K 765\nT55J5 684";
    const EXAMPLE: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_solve_part_1() {
        let result = solve_part_1(EXAMPLE).unwrap();
        let expected = 6440;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_solve_part_2() {
        let result = solve_part_2(EXAMPLE).unwrap();
        let expected = 5905;
        assert_eq!(result, expected)
    }

    #[test]
    fn test_parse_input() {
        let result = parse_input(EXAMPLE_SIMPLE, false).unwrap();
        let expected = vec![
            Hand {
                cards: vec![Three, Two, Ten, Three, King],
                strength: OnePair,
                bid: 765,
                with_joker: false,
            },
            Hand {
                cards: vec![Ten, Five, Five, Jack, Five],
                strength: ThreeOfKind,
                bid: 684,
                with_joker: false,
            },
        ];
        assert_eq!(result, expected)
    }

    #[test]
    fn test_parse_input_with_joker() {
        let result = parse_input(EXAMPLE_SIMPLE, true).unwrap();
        let expected = vec![
            Hand {
                cards: vec![Three, Two, Ten, Three, King],
                strength: OnePair,
                bid: 765,
                with_joker: true,
            },
            Hand {
                cards: vec![Ten, Five, Five, Joker, Five],
                strength: FourOfKind,
                bid: 684,
                with_joker: true,
            },
        ];
        assert_eq!(result, expected)
    }

    #[test]
    // returns vector of (bid, rank)
    fn test_get_bids_and_ranks() {
        let input = vec![
            Hand {
                cards: vec![Three, Two, Ten, Three, King],
                strength: OnePair,
                bid: 765,
                with_joker: false,
            },
            Hand {
                cards: vec![Ten, Five, Five, Jack, Five],
                strength: ThreeOfKind,
                bid: 684,
                with_joker: false,
            },
        ];
        let result = get_bids_and_ranks(&input);
        let expected = vec![(765u64, 1u64), (684u64, 2u64)];
        assert_eq!(result, expected)
    }

    #[test]
    // returns vector of (bid, rank)
    fn test_get_bids_and_ranks_with_joker() {
        let input = vec![
            Hand {
                cards: vec![Three, Two, Ten, Three, King],
                strength: OnePair,
                bid: 765,
                with_joker: true,
            },
            Hand {
                cards: vec![Ten, Five, Five, Five, Five],
                strength: FourOfKind,
                bid: 684,
                with_joker: true,
            },
        ];
        let result = get_bids_and_ranks(&input);
        let expected = vec![(765u64, 1u64), (684u64, 2u64)];
        assert_eq!(result, expected)
    }

    #[test]
    fn test_get_total_winnings() {
        // This is a vec of (bid, rank)
        let input = vec![(765u64, 1u64), (684u64, 2u64)];
        let result: u64 = get_total_winnings(&input);
        let expected = 2133;
        assert_eq!(result, expected);
    }
}

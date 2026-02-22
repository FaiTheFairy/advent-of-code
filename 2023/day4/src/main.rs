use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let sum1 = solve_part1(&input);
    println!("Part 1. Sum is {sum1}");
    let sum2 = solve_part2(&input);
    println!("Part 2. Sum is {sum2}");
}

#[derive(Eq, PartialEq, Clone)]
struct Card {
    id: usize,
    winning: HashSet<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn new(line: &str) -> Self {
        let line = line.strip_prefix("Card ").unwrap();
        let (id, all_numbers) = line.split_once(':').unwrap();
        let id: usize = id.trim().parse().unwrap();
        let (winning, numbers) = all_numbers.split_once('|').unwrap();
        let winning: HashSet<u32> = winning
            .split_whitespace()
            .filter_map(|s| s.trim().parse::<u32>().ok())
            .collect();
        let numbers: Vec<u32> = numbers
            .split_whitespace()
            .filter_map(|s| s.trim().parse::<u32>().ok())
            .collect();

        Self {
            id,
            winning,
            numbers,
        }
    }

    fn count_winning_cards(&self) -> usize {
        self.numbers
            .iter()
            .filter(|n| self.winning.contains(n))
            .count()
    }

    fn points(&self) -> usize {
        let win_count = self.count_winning_cards();
        if win_count == 0 {
            0
        } else {
            // 2^(m-1) is 1 << (m-1)
            1 << (win_count - 1)
        }
    }
}

fn solve_part1(input: &str) -> usize {
    input.lines().map(Card::new).map(|card| card.points()).sum()
}

fn solve_part2(input: &str) -> usize {
    let cards: Vec<Card> = input.lines().map(Card::new).collect();
    let n = cards.len();

    let mut counts = vec![1usize; n];

    for i in 0..n {
        let m = cards[i].count_winning_cards();
        let copies = counts[i];

        let start = i + 1;
        let end = (i + 1 + m).min(n);

        for j in start..end {
            counts[j] += copies;
        }
    }
    counts.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(EXAMPLE), 13usize);
    }
    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(EXAMPLE), 30usize);
    }
}

// i love you:)
use std::{fs, str::FromStr};

use anyhow::{Result, anyhow};

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?.parse::<Input>()?;
    let sol1 = input.score_corrupted_lines();
    println!("Part 1. total syntax score = {sol1}");

    let sol2 = input.score_completion_strings();
    println!("Part 2. middle completion score = {sol2}");
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Input(Vec<Line>);

impl Input {
    fn score_corrupted_lines(&self) -> usize {
        self.0
            .iter()
            .filter_map(|l| match l.analyze() {
                LineStatus::Corrupted(symbol) => Some(symbol),
                LineStatus::Complete | LineStatus::Incomplete(_) => None,
            })
            .map(Symbol::syntax_error_score)
            .sum()
    }

    fn score_completion_strings(&self) -> usize {
        let mut completion_scores: Vec<usize> = self
            .0
            .iter()
            .filter_map(|l| match l.analyze() {
                LineStatus::Incomplete(completion_symbols) => Some(completion_symbols),
                _ => None,
            })
            .map(|completion| {
                let mut score = 0;
                for symbol in completion {
                    score *= 5;
                    score += symbol.autocomplete_score()
                }
                score
            })
            .collect();

        completion_scores.sort_unstable();
        completion_scores[completion_scores.len() / 2]
    }
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let chunks = s
            .lines()
            .map(str::parse::<Line>)
            .collect::<Result<_, _>>()?;
        Ok(Self(chunks))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Line(Vec<Delimiter>);

impl Line {
    fn analyze(&self) -> LineStatus {
        let mut stack = Vec::new();

        for d in &self.0 {
            match d {
                Delimiter::Opening(symbol) => stack.push(*symbol),
                Delimiter::Closing(symbol) => {
                    let expected = stack.pop();
                    if expected != Some(*symbol) {
                        return LineStatus::Corrupted(*symbol);
                    }
                }
            }
        }
        if stack.is_empty() {
            LineStatus::Complete
        } else {
            stack.reverse();
            LineStatus::Incomplete(stack)
        }
    }
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let delimiters = s
            .trim()
            .chars()
            .map(Delimiter::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(delimiters))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum LineStatus {
    Complete,
    Corrupted(Symbol),
    Incomplete(Vec<Symbol>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Delimiter {
    Opening(Symbol),
    Closing(Symbol),
}

// you are so beautiful my princess
impl TryFrom<char> for Delimiter {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        use Delimiter::*;
        use Symbol::*;
        match value {
            '(' => Ok(Opening(Parentheses)),
            ')' => Ok(Closing(Parentheses)),

            '[' => Ok(Opening(Square)),
            ']' => Ok(Closing(Square)),

            '{' => Ok(Opening(Curly)),
            '}' => Ok(Closing(Curly)),

            '<' => Ok(Opening(Angled)),
            '>' => Ok(Closing(Angled)),
            _ => Err(anyhow!("unknown character: {value}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Symbol {
    /// `(` or `)`
    Parentheses,
    /// `[` or `]`
    Square,
    /// `{` or `}`
    Curly,
    /// `<` or `>`
    Angled,
}

impl Symbol {
    fn syntax_error_score(self) -> usize {
        match self {
            Symbol::Parentheses => 3,
            Symbol::Square => 57,
            Symbol::Curly => 1197,
            Symbol::Angled => 25137,
        }
    }
    fn autocomplete_score(self) -> usize {
        match self {
            Symbol::Parentheses => 1,
            Symbol::Square => 2,
            Symbol::Curly => 3,
            Symbol::Angled => 4,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_score_completion() {
        let result = EXAMPLE.parse::<Input>().unwrap().score_completion_strings();
        assert_eq!(result, 288_957);
    }

    #[test]
    fn test_score_syntax_error() {
        let result = EXAMPLE.parse::<Input>().unwrap().score_corrupted_lines();
        assert_eq!(result, 26397);
    }

    #[test]
    fn test_analyze_line() {
        let result = "((>))".parse::<Line>().unwrap().analyze();
        assert_eq!(result, LineStatus::Corrupted(Symbol::Angled));

        let result = "({[<>]})".parse::<Line>().unwrap().analyze();
        assert_eq!(result, LineStatus::Complete);

        let result = "(())<<".parse::<Line>().unwrap().analyze();
        assert_eq!(
            result,
            LineStatus::Incomplete(vec![Symbol::Angled, Symbol::Angled])
        );
    }

    #[test]
    fn test_parse_delimiter() {
        let paren_open: Delimiter = '('.try_into().unwrap();
        let paren_closed: Delimiter = ')'.try_into().unwrap();

        let square_open: Delimiter = '['.try_into().unwrap();
        let square_closed: Delimiter = ']'.try_into().unwrap();

        let curly_open: Delimiter = '{'.try_into().unwrap();
        let curly_closed: Delimiter = '}'.try_into().unwrap();

        let angled_open: Delimiter = '<'.try_into().unwrap();
        let angled_closed: Delimiter = '>'.try_into().unwrap();

        use Delimiter::*;
        use Symbol::*;
        assert_eq!(paren_open, Opening(Parentheses));
        assert_eq!(paren_closed, Closing(Parentheses));

        assert_eq!(square_open, Opening(Square));
        assert_eq!(square_closed, Closing(Square));

        assert_eq!(curly_open, Opening(Curly));
        assert_eq!(curly_closed, Closing(Curly));

        assert_eq!(angled_open, Opening(Angled));
        assert_eq!(angled_closed, Closing(Angled));
    }

    #[test]
    fn test_parse_line() {
        use Delimiter::*;
        use Symbol::*;

        let result = "([<(})])".parse::<Line>().unwrap();
        let expected = Line(vec![
            Opening(Parentheses),
            Opening(Square),
            Opening(Angled),
            Opening(Parentheses),
            Closing(Curly),
            Closing(Parentheses),
            Closing(Square),
            Closing(Parentheses),
        ]);
        assert_eq!(result, expected);
    }

    // mi amante apasionado
    #[test]
    fn test_parse_input() {
        let result = "((\n))\n<>\n{{()}}".parse::<Input>().unwrap();
        let expected = Input(vec![
            "((".parse::<Line>().unwrap(),
            "))".parse::<Line>().unwrap(),
            "<>".parse::<Line>().unwrap(),
            "{{()}}".parse::<Line>().unwrap(),
        ]);
        assert_eq!(result, expected);
    }
}

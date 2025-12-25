#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

fn parse_input(input: &str) -> Vec<Point> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        let result = parse_input(input);
        let expected = vec![
            Point { x: 7, y: 1 },
            Point { x: 11, y: 1 },
            Point { x: 11, y: 7 },
            Point { x: 9, y: 7 },
            Point { x: 9, y: 5 },
            Point { x: 2, y: 5 },
            Point { x: 2, y: 3 },
            Point { x: 7, y: 3 },
        ];
        assert_eq!(result, expected);
    }
}

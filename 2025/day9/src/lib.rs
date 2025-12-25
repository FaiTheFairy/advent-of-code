#[derive(Debug, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

fn parse_input(input: &str) -> Vec<Point> {
    let mut points: Vec<Point> = vec![];
    for line in input.lines() {
        let mut iterator = line.splitn(2, ',');
        let x: i64 = iterator.next().unwrap().parse().unwrap();
        let y: i64 = iterator.next().unwrap().parse().unwrap();
        points.push(Point { x, y });
    }
    points
}

fn max_area(points: &[Point]) -> i64 {
    let mut max_area = 0;
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let p1 = &points[i];
            let p2 = &points[j];
            // Area is counted in number of tiles, not geometric units.
            // Since both corner tiles are included in the rectangle,
            // the width and height are inclusive: (|dx| + 1) × (|dy| + 1).
            let dx = (p1.x - p2.x).abs() + 1;
            let dy = (p1.y - p2.y).abs() + 1;
            let area = dx * dy;
            max_area = max_area.max(area);
        }
    }
    max_area
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_parse_input() {
        let result = parse_input(INPUT);
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

    #[test]
    fn test_max_area() {
        let points = parse_input(INPUT);
        let result = max_area(&points);
        let expected: i64 = 50;
        assert_eq!(result, expected);
    }
}

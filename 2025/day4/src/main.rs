use std::{fs::File, io::Read, path::PathBuf};

#[derive(Debug, Clone)]
struct Point {
    x: usize,
    y: usize,
    occupied: bool,
}

fn parse_grid(input: &str) -> Vec<Point> {
    let mut points_vec = vec![];
    let lines = input.lines();
    for (y, line) in lines.enumerate() {
        for (x, char) in line.chars().enumerate() {
            let point = Point {
                x,
                y,
                occupied: char == '@',
            };
            points_vec.push(point);
        }
    }
    points_vec
}

fn count_adjacent_occupied(point: &Point, points: &[Point]) -> usize {
    points
        .iter()
        .filter(|p| {
            let dx = p.x.abs_diff(point.x);
            let dy = p.y.abs_diff(point.y);
            (dx <= 1 && dy <= 1) && !(dx == 0 && dy == 0) && p.occupied
        })
        .count()
}

fn main() {
    let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let mut file = File::open(filename).expect("Cannot open file");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    let grid = parse_grid(&buffer);
    let mut count_accessible = 0;
    for point in &grid {
        if point.occupied && count_adjacent_occupied(point, &grid) < 4 {
            count_accessible += 1;
        }
    }
    println!("Accessible points: {count_accessible}");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_count_adjacent_occupied() {
        let point1 = Point {
            x: 0,
            y: 0,
            occupied: true,
        };
        let point2 = Point {
            x: 1,
            y: 0,
            occupied: false,
        };
        let point3 = Point {
            x: 2,
            y: 0,
            occupied: false,
        };
        let point4 = Point {
            x: 3,
            y: 0,
            occupied: false,
        };
        let point5 = Point {
            x: 0,
            y: 1,
            occupied: true,
        };
        let point6 = Point {
            x: 1,
            y: 1,
            occupied: false,
        };
        let point7 = Point {
            x: 2,
            y: 1,
            occupied: false,
        };
        let point8 = Point {
            x: 3,
            y: 1,
            occupied: false,
        };
        let point9 = Point {
            x: 0,
            y: 2,
            occupied: true,
        };
        let point10 = Point {
            x: 1,
            y: 2,
            occupied: true,
        };
        let point11 = Point {
            x: 2,
            y: 2,
            occupied: false,
        };
        let point12 = Point {
            x: 3,
            y: 2,
            occupied: false,
        };
        let point_vec = vec![
            point1,
            point2,
            point3,
            point4,
            point5,
            point6.clone(),
            point7,
            point8,
            point9,
            point10,
            point11,
            point12,
        ];
        let count = count_adjacent_occupied(point6, &point_vec);
        let expected = 4usize;
        assert_eq!(count, expected);
    }

    #[test]
    fn test_part_1() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        let points_vec = parse_grid(input);
        let mut count_accessible = 0;
        for point in points_vec.clone() {
            if point.occupied && count_adjacent_occupied(point, &points_vec) < 4 {
                count_accessible += 1;
            }
        }
        assert_eq!(count_accessible, 13);
    }
}

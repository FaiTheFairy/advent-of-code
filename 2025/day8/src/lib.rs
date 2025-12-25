mod dsu;
use dsu::Dsu;

#[derive(Debug, PartialEq)]
pub struct Point {
    x: i64,
    y: i64,
    z: i64,
}

/// Parses input in the form of "1,2,3\n1,3,3" into vector of Point structs
pub fn parse_input(input: &str) -> Vec<Point> {
    let mut points: Vec<Point> = vec![];
    for line in input.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        let [x, y, z] = parts.as_slice() else {
            panic!("Invalid coordinate line: {line}")
        };
        let x: i64 = x.parse().unwrap();
        let y: i64 = y.parse().unwrap();
        let z: i64 = z.parse().unwrap();
        points.push(Point { x, y, z })
    }
    points
}

/// Calculates the squared distance between two 3-dimensional points
fn squared_distance(a: &Point, b: &Point) -> i128 {
    (a.x - b.x).pow(2) as i128 + (a.y - b.y).pow(2) as i128 + (a.z - b.z).pow(2) as i128
}

/// Generate all unique pairwise edges between junction boxes.
///
/// Each edge represents a possible connection between two distinct junction
/// boxes `i` and `j` (with `i < j`).
///
/// The returned vector contains tuples of the form:
/// `(distance_squared, i, j)`, where `i` and `j` are indices into the input
/// `points` slice.
///
/// Squared distance is used instead of true Euclidean distance because only
/// relative ordering matters, and this avoids floating-point computation.
fn all_edges(points: &[Point]) -> Vec<(i128, usize, usize)> {
    let mut edges: Vec<(i128, usize, usize)> = vec![];
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let d2 = squared_distance(&points[i], &points[j]);
            edges.push((d2, i, j));
        }
    }
    edges
}

pub fn connect_n_closest(points: &[Point], n: usize) -> Dsu {
    let mut edges = all_edges(points);
    edges.sort_unstable_by_key(|(d2, _, _)| *d2);

    let mut dsu = Dsu::new(points.len());

    for &(_, i, j) in edges.iter().take(n) {
        dsu.union(i, j);
    }

    dsu
}

pub fn component_sizes(dsu: &mut Dsu) -> Vec<usize> {
    let n = dsu.parent.len();

    // Ensure path compression is applied everywhere
    for i in 0..n {
        dsu.find(i);
    }

    let mut sizes: Vec<usize> = vec![];

    for i in 0..n {
        if dsu.parent[i] == i {
            // i is a root
            sizes.push(dsu.size[i]);
        }
    }

    sizes
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940";
    #[test]
    fn test_parse_input() {
        let result = parse_input(INPUT);
        let expected = vec![
            Point {
                x: 162,
                y: 817,
                z: 812,
            },
            Point {
                x: 57,
                y: 618,
                z: 57,
            },
            Point {
                x: 906,
                y: 360,
                z: 560,
            },
            Point {
                x: 592,
                y: 479,
                z: 940,
            },
        ];

        assert_eq!(result, expected);
    }
}

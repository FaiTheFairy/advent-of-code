struct Point {
    x: i64,
    y: i64,
    z: i64,
}

fn parse_input(input: &str) -> Vec<Point> {
    let points: Vec<Point> = vec![];
    for line in input.lines() {
        let parts: Vec<&str> = input.split(',').collect();
        let [x, y, z] = parts.as_slice() else {
            panic!("Invalid coordinate line: {line}")
        };
        let x: i64 = x.parse().unwrap();
        let y: i64 = y.parse().unwrap();
        let z: i64 = z.parse().unwrap();
    }
    points
}

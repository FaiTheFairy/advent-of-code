use anyhow::{Context, Result};

fn main() -> Result<()> {
    let input = "Time:        49     97     94     94
Distance:   263   1532   1378   1851";
    let soln1 = solve_day1(input)?;
    println!("Part 1. Answer = {soln1}");

    let soln2 = solve_day2(input)?;
    println!("Part 2. Answer = {soln2}");
    Ok(())
}

fn solve_day1(input: &str) -> Result<usize> {
    let race_records = parse_race_records(input)?;
    let product = race_records
        .iter()
        // .map(RaceRecord::ways_to_win_brute)
        .map(RaceRecord::ways_to_win)
        .product();
    Ok(product)
}

fn solve_day2(input: &str) -> Result<usize> {
    let race_record: RaceRecord = parse_race_records_combined(input)?;
    // let number_of_ways = race_record.ways_to_win_brute();
    let number_of_ways = race_record.ways_to_win();
    Ok(number_of_ways)
}

fn parse_race_records_combined(input: &str) -> Result<RaceRecord> {
    let mut lines = input.lines();
    let time = lines
        .next()
        .context("can't find time line")?
        .strip_prefix("Time: ")
        .context("line doesn't start with 'Time: '")?
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<usize>()?;
    let distance = lines
        .next()
        .context("can't find distance line")?
        .strip_prefix("Distance: ")
        .context("line doesn't start with 'Distance: '")?
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<usize>()?;
    Ok(RaceRecord { time, distance })
}

fn parse_race_records(input: &str) -> Result<Vec<RaceRecord>> {
    let mut lines = input.lines();
    let times_iter = lines
        .next()
        .context("No time to parse")?
        .split_whitespace()
        .skip(1)
        .map(|t| t.parse::<usize>());
    let distance_iter = lines
        .next()
        .context("No distance to parse")?
        .split_whitespace()
        .skip(1)
        .map(|t| t.parse::<usize>());

    let mut out = Vec::new();
    for (time, distance) in times_iter.zip(distance_iter) {
        let time = time?;
        let distance = distance?;
        out.push(RaceRecord { time, distance });
    }
    Ok(out)
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct RaceRecord {
    time: usize,
    distance: usize,
}

impl RaceRecord {
    #[allow(dead_code)]
    fn ways_to_win_brute(&self) -> usize {
        let mut count = 0;

        for h in 1..self.time {
            let distance = h * (self.time - h);
            if distance > self.distance {
                count += 1;
            }
        }

        count
    }

    /// Count integer h where h*(t-h) > d, with 0 <= h <= t.
    /// Solve h^2 - t h + d < 0. The solution is integers strictly between the roots
    #[allow(dead_code)]
    fn ways_to_win(&self) -> usize {
        let t = self.time;
        let d = self.distance;
        let disc = (t as f64) * (t as f64) - 4.0 * (d as f64);
        if disc <= 0.0 {
            return 0;
        }

        let s = disc.sqrt();
        let r1 = ((t as f64) - s) / 2.0;
        let r2 = ((t as f64) + s) / 2.0;

        // integers strictly inside (r1, r2)
        let lo = r1.floor() as i64 + 1;
        let hi = r2.ceil() as i64 - 1;

        (hi - lo + 1).max(0) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_solve_day1() {
        let result = solve_day1(EXAMPLE).unwrap();
        let expected = 288;
        assert_eq!(result, expected)
    }

    #[test]
    fn test_solve_day2() {
        let result = solve_day2(EXAMPLE).unwrap();
        let expected: usize = 71503;
        assert_eq!(result, expected)
    }

    #[test]
    fn test_parse_race_records() {
        let result = parse_race_records(EXAMPLE).unwrap();
        let expected = vec![
            RaceRecord {
                time: 7,
                distance: 9,
            },
            RaceRecord {
                time: 15,
                distance: 40,
            },
            RaceRecord {
                time: 30,
                distance: 200,
            },
        ];
        assert_eq!(result, expected)
    }
}

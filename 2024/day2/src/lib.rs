pub fn parse_input(input: &str) -> Vec<Vec<u64>> {
    let mut reports = vec![];
    for line in input.lines() {
        let levels: Vec<u64> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        reports.push(levels);
    }
    reports
}

fn is_safe(levels: &[u64]) -> bool {
    let increasing = levels[1] > levels[0];
    for i in 0..(levels.len() - 1) {
        let is_monotone = (increasing && (levels[i] < levels[i + 1]))
            || (!increasing && (levels[i] > levels[i + 1]));
        let is_diff_within_range =
            1 <= levels[i].abs_diff(levels[i + 1]) && levels[i].abs_diff(levels[i + 1]) <= 3;
        if !is_monotone || !is_diff_within_range {
            return false;
        }
    }
    true
}

fn is_safe_with_damper(levels: &[u64]) -> bool {
    if is_safe(levels) {
        return true;
    }

    // try removing exactly one level
    for skip in 0..levels.len() {
        let mut reduced: Vec<u64> = Vec::with_capacity(levels.len() - 1);
        for (idx, &v) in levels.iter().enumerate() {
            if idx != skip {
                reduced.push(v);
            }
        }
        if is_safe(&reduced) {
            return true;
        }
    }

    false
}

pub fn count_safe(reports: &[Vec<u64>]) -> u64 {
    let mut count = 0;
    for levels in reports {
        if is_safe(levels) {
            count += 1;
        }
    }
    count
}

pub fn count_safe_with_damper(reports: &[Vec<u64>]) -> u64 {
    let mut count = 0;
    for levels in reports {
        if is_safe_with_damper(levels) {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_parse_input() {
        let result = parse_input(EXAMPLE);
        let expected = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_is_safe_increasing() {
        let result = is_safe(&[0, 1, 2, 3, 4]);
        assert!(result);
    }

    #[test]
    fn test_is_safe_decreasing() {
        let result = is_safe(&[4, 3, 2, 1, 0]);
        assert!(result)
    }

    #[test]
    fn test_is_safe_non_monotone() {
        let result = is_safe(&[1, 2, 3, 4, 3, 2, 1]);
        assert!(!result)
    }

    #[test]
    fn test_is_safe_equal() {
        let result = is_safe(&[0, 0, 1, 2, 3, 4]);
        assert!(!result);
    }

    #[test]
    fn test_is_safe_not_within_range() {
        let result = is_safe(&[0, 1, 2, 6, 8]);
        assert!(!result);
    }

    #[test]
    fn test_count_is_safe() {
        let reports = parse_input(EXAMPLE);
        let count_is_safe = count_safe(&reports);
        assert_eq!(count_is_safe, 2);
    }

    #[test]
    fn test_count_is_safe_with_damper() {
        let reports = parse_input(EXAMPLE);
        let count_safe_with_damper = count_safe_with_damper(&reports);
        assert_eq!(count_safe_with_damper, 4);
    }
}

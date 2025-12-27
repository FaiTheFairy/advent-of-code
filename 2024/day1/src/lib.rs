use std::collections::HashMap;

pub fn parse_input(input: &str) -> (Vec<u64>, Vec<u64>) {
    let mut list1 = vec![];
    let mut list2 = vec![];
    for line in input.lines() {
        let (num1, num2) = line
            .split_once(' ')
            .expect("expected space in input: {line}");
        list1.push(num1.trim().parse().unwrap());
        list2.push(num2.trim().parse().unwrap());
    }
    list1.sort();
    list2.sort();
    (list1, list2)
}

pub fn compute_diffs(list1: &[u64], list2: &[u64]) -> Vec<u64> {
    let mut diff: Vec<u64> = vec![];
    for (i, j) in list1.iter().zip(list2.iter()) {
        diff.push(i.abs_diff(*j));
    }
    diff
}

pub fn compute_similarity(list1: &[u64], list2: &[u64]) -> u64 {
    let mut similarity_score: u64 = 0;
    let mut freq: HashMap<u64, u64> = HashMap::new();

    // iterate through list2 and record how many times each value is repeated
    for x in list2 {
        *freq.entry(*x).or_insert(0) += 1;
    }

    for x in list1 {
        similarity_score += x * freq.get(x).unwrap_or(&0);
    }

    similarity_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";

        let result = parse_input(input);
        let expected = (vec![1, 2, 3, 3, 3, 4], vec![3, 3, 3, 4, 5, 9]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_compute_diffs() {
        let lists = (vec![1, 2, 3, 3, 3, 4], vec![3, 3, 3, 4, 5, 9]);
        let result = compute_diffs(&lists.0, &lists.1);
        let expected: Vec<u64> = vec![2, 1, 0, 1, 2, 5];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_compute_similarity() {
        let lists = (vec![1, 2, 3, 3, 3, 4], vec![3, 3, 3, 4, 5, 9]);
        let result = compute_similarity(&lists.0, &lists.1);
        let expected: u64 = 31;
        assert_eq!(result, expected);
    }
}

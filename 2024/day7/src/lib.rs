#[derive(Debug, PartialEq)]
pub enum Operators {
    Add,
    Mult,
}

#[derive(Debug, PartialEq)]
pub struct Equation {
    target: u64,
    nums: Vec<u64>,
}

pub fn parse_input(input: &str) -> Vec<Equation> {
    let mut equations = vec![];
    for line in input.lines() {
        let Some((target, values)) = line.split_once(':') else {
            panic!("Invalid line: {line}")
        };

        let target: u64 = target.parse().unwrap();
        let mut nums: Vec<u64> = vec![];
        for num in values.split_whitespace() {
            nums.push(num.parse().unwrap());
        }
        equations.push(Equation { target, nums });
    }
    equations
}

fn can_make_target(nums: &[u64], target: u64) -> bool {
    fn dfs(nums: &[u64], target: u64, idx: usize, acc: u64) -> bool {
        if idx == nums.len() {
            return acc == target;
        }

        if acc > target {
            return false;
        }

        let x = nums[idx];

        dfs(nums, target, idx + 1, acc + x) || dfs(nums, target, idx + 1, acc * x)
    }

    dfs(nums, target, 1, nums[0])
}

#[allow(dead_code)]
fn is_valid(eq: &Equation) -> bool {
    can_make_target(&eq.nums, eq.target)
}

pub fn total_calibration(eqs: &[Equation]) -> u64 {
    eqs.iter()
        .filter(|e| can_make_target(&e.nums, e.target))
        .map(|e| e.target)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_parse_input() {
        let result = parse_input(EXAMPLE);
        let expected = vec![
            Equation {
                target: 190,
                nums: vec![10, 19],
            },
            Equation {
                target: 3267,
                nums: vec![81, 40, 27],
            },
            Equation {
                target: 83,
                nums: vec![17, 5],
            },
            Equation {
                target: 156,
                nums: vec![15, 6],
            },
            Equation {
                target: 7290,
                nums: vec![6, 8, 6, 15],
            },
            Equation {
                target: 161011,
                nums: vec![16, 10, 13],
            },
            Equation {
                target: 192,
                nums: vec![17, 8, 14],
            },
            Equation {
                target: 21037,
                nums: vec![9, 7, 18, 13],
            },
            Equation {
                target: 292,
                nums: vec![11, 6, 16, 20],
            },
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_do_operation() {
        let valid_mult = Equation {
            target: 20,
            nums: vec![2, 5, 2],
        };
        assert!(is_valid(&valid_mult));

        let valid_add = Equation {
            target: 9,
            nums: vec![2, 5, 2],
        };
        assert!(is_valid(&valid_add));

        let invalid = Equation {
            target: 100,
            nums: vec![5, 3, 4, 1],
        };
        assert!(!is_valid(&invalid));
    }

    #[test]
    fn test_solve_part1() {
        let equations = parse_input(EXAMPLE);
        assert_eq!(total_calibration(&equations), 3749u64);
    }
}

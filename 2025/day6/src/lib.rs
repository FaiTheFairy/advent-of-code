#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Addition,
    Multiplication,
}

pub fn parse_input(input: &str) -> Vec<(Vec<usize>, Operation)> {
    let mut row_0: Vec<usize> = vec![];
    let mut row_1: Vec<usize> = vec![];
    let mut row_2: Vec<usize> = vec![];
    let mut row_3: Vec<usize> = vec![];
    let mut operators = vec![];

    for (idx, line) in input.lines().enumerate() {
        for value in line.split_whitespace() {
            match idx {
                0 => row_0.push(value.parse().unwrap()),
                1 => row_1.push(value.parse().unwrap()),
                2 => row_2.push(value.parse().unwrap()),
                3 => row_3.push(value.parse().unwrap()),
                4 => match value.trim() {
                    "+" => operators.push(Operation::Addition),
                    "*" => operators.push(Operation::Multiplication),
                    _ => panic!("Invalid operator \"{}\"!", value.trim()),
                },
                _ => panic!("Only 4 rows valid!"),
            }
        }
    }

    let mut out: Vec<(Vec<usize>, Operation)> = vec![];
    for ((((num1, num2), num3), num4), operator) in row_0
        .iter()
        .zip(&row_1)
        .zip(&row_2)
        .zip(&row_3)
        .zip(&operators)
    {
        out.push((vec![*num1, *num2, *num3, *num4], *operator));
    }
    out
}

pub fn parse_input_rtl(input: &str) -> Vec<(Vec<usize>, Operation)> {
    let mut row_0: Vec<usize> = vec![];
    let mut row_1: Vec<usize> = vec![];
    let mut row_2: Vec<usize> = vec![];
    let mut row_3: Vec<usize> = vec![];
    let mut operators = vec![];

    for (idx, line) in input.lines().enumerate() {
        for value in line.split_whitespace() {
            match idx {
                0 => row_0.push(value.parse().unwrap()),
                1 => row_1.push(value.parse().unwrap()),
                2 => row_2.push(value.parse().unwrap()),
                3 => row_3.push(value.parse().unwrap()),
                4 => match value.trim() {
                    "+" => operators.push(Operation::Addition),
                    "*" => operators.push(Operation::Multiplication),
                    _ => panic!("Invalid operator \"{}\"!", value.trim()),
                },
                _ => panic!("Only 4 rows valid!"),
            }
        }
    }

    let mut out: Vec<(Vec<usize>, Operation)> = vec![];
    for ((((num1, num2), num3), num4), operator) in row_0
        .iter()
        .zip(&row_1)
        .zip(&row_2)
        .zip(&row_3)
        .zip(&operators)
    {
        out.push((vec![*num1, *num2, *num3, *num4], *operator));
    }
    out
}

pub fn calculate_columns(columns: Vec<(Vec<usize>, Operation)>) -> Vec<usize> {
    let mut out: Vec<usize> = vec![];
    for column in columns {
        match column.1 {
            Operation::Addition => {
                let mut result = 0;
                for val in column.0 {
                    result += val;
                }
                out.push(result);
            }
            Operation::Multiplication => {
                let mut result = 1;
                for val in column.0 {
                    result *= val;
                }
                out.push(result);
            }
        }
    }
    out
}

pub fn calculate_sum_of_results(results: Vec<usize>) -> usize {
    results.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use Operation::*;

    const EXAMPLE: &str = "1 2 3\n1 2 3\n1 2 3\n1 2 3\n+ * *";
    const EXAMPLE_PART1: &str = "123 328  51 64
         45  64  387 23
          6  98  215 314
          1   0   1     0
         *   +   *   +   ";
    #[test]
    fn test_parse_input_simple() {
        let expected = vec![
            (vec![1, 1, 1, 1], Addition),
            (vec![2, 2, 2, 2], Multiplication),
            (vec![3, 3, 3, 3], Multiplication),
        ];
        let result = parse_input(EXAMPLE);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_input_part1_modified_to_4_rows() {
        let expected = vec![
            (vec![123, 45, 6, 1], Multiplication),
            (vec![328, 64, 98, 0], Addition),
            (vec![51, 387, 215, 1], Multiplication),
            (vec![64, 23, 314, 0], Addition),
        ];
        let result = parse_input(EXAMPLE_PART1);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_input_rtl_part1() {
        let expected = vec![
            (vec![14, 2561, 3], Multiplication),
            (vec![369, 2480, 8], Addition),
            (vec![32, 5811, 175], Multiplication),
            (vec![623, 431, 4, 0], Addition),
        ];
        let result = parse_input_rtl(EXAMPLE_PART1);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_calculate_columns_simple() {
        let columns = vec![
            (vec![1, 1, 1], Addition),
            (vec![2, 2, 2], Multiplication),
            (vec![3, 3, 3], Multiplication),
        ];
        let result = calculate_columns(columns);
        let expected = vec![3, 8, 27];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_calculate_columns_part1() {
        let columns = vec![
            (vec![123, 45, 6], Multiplication),
            (vec![328, 64, 98], Addition),
            (vec![51, 387, 215], Multiplication),
            (vec![64, 23, 314], Addition),
        ];
        let result = calculate_columns(columns);
        let expected = vec![33_210, 490, 4_243_455, 401];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_calculate_sum_results() {
        let results = vec![33_210, 490, 4_243_455, 401];
        let sum = calculate_sum_of_results(results);
        let expected = 4_277_556;
        assert_eq!(sum, expected);
    }
}

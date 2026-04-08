use anyhow::Result;
use serde_json::Value;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.json")?;

    let sol1 = sum_numbers(&input);
    println!("Part 1: {sol1}");

    let value: Value = serde_json::from_str(&input)?;
    let sol2 = sum_json(&value, "red");
    println!("Part 2: {sol2}");

    Ok(())
}

fn sum_numbers(s: &str) -> i64 {
    let mut sum = 0;
    let mut current = 0;
    let mut sign = 1;
    let mut in_number = false;

    for b in s.bytes() {
        match b {
            b'-' => {
                sign = -1;
                in_number = true;
                current = 0;
            }
            b'0'..=b'9' => {
                current = current * 10 + (b - b'0') as i64;
                in_number = true;
            }
            _ => {
                if in_number {
                    sum += sign * current;
                    current = 0;
                    sign = 1;
                    in_number = false;
                }
            }
        }
    }

    if in_number {
        sum += sign * current;
    }
    sum
}

fn sum_json(value: &Value, exclude: &str) -> i64 {
    match value {
        Value::Number(number) => number.as_i64().unwrap_or(0),
        Value::Array(values) => values.iter().map(|v| sum_json(v, exclude)).sum(),
        Value::Object(map) => {
            if map
                .values()
                .any(|v| matches!(v, Value::String(s) if s == exclude))
            {
                0
            } else {
                map.values().map(|v| sum_json(v, exclude)).sum()
            }
        }
        Value::Null | Value::String(_) | Value::Bool(_) => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_numbers() {
        assert_eq!(sum_numbers("[1,2,3]"), 6);
        assert_eq!(sum_numbers(r#"{"a":2,"b":4}"#), 6);

        assert_eq!(sum_numbers("[[[3]]]"), 3);
        assert_eq!(sum_numbers(r#""a":{"b":4},"c":-1}"#), 3);

        assert_eq!(sum_numbers(r#"{"a":[-1,1]}"#), 0);
        assert_eq!(sum_numbers(r#"[-1,{"a":1}]"#), 0);

        assert_eq!(sum_numbers("[]"), 0);
        assert_eq!(sum_numbers("{}"), 0);
    }
}

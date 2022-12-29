//! # Solution for Advent of Code 2015 Day 12:
//!
//! Ref: [Advent of Code 2015 Day 12](https://adventofcode.com/2015/day/12)
//!
use anyhow::Result;
use serde_json::Value;
use std::io::{self, Read};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Condition {
    Unconditional,
    IgnoreRedMarks,
}

fn sum_numbers(v: &Value, condition: Condition) -> i64 {
    match v {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(x) => x.as_i64().expect("input file should not contain floats"),
        Value::String(_) => 0,
        Value::Array(ary) => ary.iter().map(|v| sum_numbers(v, condition)).sum(),
        Value::Object(map) => {
            if condition == Condition::IgnoreRedMarks
                && map
                    .values()
                    .any(|val| val.as_str().map(|s| s == "red").unwrap_or(false))
            {
                0
            } else {
                map.values().map(|v| sum_numbers(v, condition)).sum()
            }
        }
    }
}

fn core(input: &str, cond: Condition) -> Result<i64> {
    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(input)?;
    Ok(sum_numbers(&v, cond))
}

fn part1(input: &str) -> Result<i64> {
    core(input, Condition::Unconditional)
}

fn part2(input: &str) -> Result<i64> {
    core(input, Condition::IgnoreRedMarks)
}

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;

    println!("Part1: {}", part1(&input)?);
    println!("Part2: {}", part2(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("[1,2,3]" => 6; "array")]
    #[test_case(r#"{"a":2,"b":4}"# => 6; "object")]
    #[test_case("[[[3]]]" => 3; "nested array")]
    #[test_case(r#"{"a":{"b":4},"c":-1}"# => 3; "nested object")]
    #[test_case(r#"{"a":[-1,1]}"# => 0; "array in object")]
    #[test_case(r#"[-1,{"a":1}]"# => 0; "object in array")]
    #[test_case("[]" => 0; "empty array")]
    #[test_case("{}" => 0; "empty object")]
    fn sum_numbers(s: &str) -> i64 {
        let v: Value = serde_json::from_str(s).unwrap();
        super::sum_numbers(&v, Condition::Unconditional)
    }

    #[test_case("[1,2,3]" => 6; "non-triggering array")]
    #[test_case(r#"[1,{"c":"red","b":2},3]"# => 4; "triggering array")]
    #[test_case(r#"{"d":"red","e":[1,2,3,4],"f":5}"# => 0; "triggering object")]
    #[test_case(r#"[1,"red",5]"# => 6; "flag in array values")]
    fn sum_nonred_numbers(s: &str) -> i64 {
        let v: Value = serde_json::from_str(s).unwrap();
        super::sum_numbers(&v, Condition::IgnoreRedMarks)
    }
}

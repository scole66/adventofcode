use std::io::{self, BufRead};

fn literal_size(s: &str) -> usize {
    s.len()
}

fn value_size(s: &str) -> anyhow::Result<usize> {
    #[derive(Debug)]
    enum ScanState {
        LeadingQuote,
        CharStart,
        EscapeStart,
        FirstHex,
        SecondHex,
    }
    let mut state = ScanState::LeadingQuote;
    let mut value_len = 0;
    for ch in s.chars() {
        match state {
            ScanState::LeadingQuote => {
                if ch == '"' {
                    state = ScanState::CharStart;
                } else {
                    anyhow::bail!("Didn't see leading double-quote");
                }
            }
            ScanState::CharStart => {
                if ch == '"' {
                    return Ok(value_len);
                } else if ch == '\\' {
                    state = ScanState::EscapeStart;
                } else {
                    value_len += 1;
                }
            }
            ScanState::EscapeStart => {
                if ch == '\\' || ch == '"' {
                    value_len += 1;
                    state = ScanState::CharStart;
                } else if ch == 'x' {
                    state = ScanState::FirstHex;
                } else {
                    anyhow::bail!("Blown escape sequence (saw '{ch}')")
                }
            }
            ScanState::FirstHex => {
                if ch.is_ascii_hexdigit() {
                    state = ScanState::SecondHex;
                } else {
                    anyhow::bail!("Blown hex escape sequence")
                }
            }
            ScanState::SecondHex => {
                if ch.is_ascii_hexdigit() {
                    value_len += 1;
                    state = ScanState::CharStart;
                } else {
                    anyhow::bail!("Blown hex escape sequence")
                }
            }
        }
    }
    Ok(value_len)
}

fn encode(s: &str) -> String {
    let mut result = String::from('"');
    for ch in s.chars() {
        match ch {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            _ => result.push(ch),
        }
    }
    result.push('"');
    result
}

fn part1(input: &[String]) -> anyhow::Result<usize> {
    Ok(input
        .iter()
        .map(|s| Ok(literal_size(s) - value_size(s)?))
        .collect::<Result<Vec<usize>, anyhow::Error>>()?
        .iter()
        .sum())
}

fn part2(input: &[String]) -> usize {
    input.iter().map(|s| encode(s).len() - literal_size(s)).sum()
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();

    let input = stdin.lock().lines().collect::<Result<Vec<_>, std::io::Error>>()?;

    println!("Part1: {}", part1(&input)?);
    println!("Part2: {}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &[&str] = &["\"\"", "\"abc\"", "\"aaa\\\"aaa\"", "\"\\x27\""];

    #[test]
    fn sample_part1() {
        let lines = SAMPLE.iter().map(|&s| s.to_string()).collect::<Vec<_>>();
        assert_eq!(part1(&lines).unwrap(), 12);
    }

    #[test]
    fn sample_part2() {
        let lines = SAMPLE.iter().map(|&s| s.to_string()).collect::<Vec<_>>();
        assert_eq!(part2(&lines), 19);
    }
}

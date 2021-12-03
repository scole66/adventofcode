use std::io;

fn choose_greater(number_zeroes: u32, number_ones: u32) -> u32 {
    if number_zeroes > number_ones {
        0
    } else {
        1
    }
}
fn choose_smaller(number_zeroes: u32, number_ones: u32) -> u32 {
    if number_zeroes <= number_ones {
        0
    } else {
        1
    }
}

fn digit_at_idx(line: &str, idx: usize) -> u8 {
    line.as_bytes()[idx]
}

fn digit_counts(lines: &[String], column: usize) -> (u32, u32) {
    lines
        .iter()
        .map(|s| digit_at_idx(s, column))
        .fold((0_u32, 0_u32), |(nz, no), ch| {
            (
                if ch == 0x30 { nz + 1 } else { nz },
                if ch == 0x31 { no + 1 } else { no },
            )
        })
}

fn digit_filter(lines: &[String], chooser: fn(u32, u32) -> u32) -> u64 {
    let mut result = 0;
    let digits = lines[0].len();
    for idx in 0..digits {
        let (nz, no) = digit_counts(lines, idx);
        let newbit = chooser(nz, no) as u64;
        result = result * 2 + newbit;
    }

    result
}

fn value_reducer(lines: &[String], starting_index: usize, chooser: fn(u32, u32) -> u32) -> u64 {
    if lines.len() == 1 {
        return u64::from_str_radix(lines[0].as_str(), 2).unwrap();
    }

    // More than one line, so filter.
    let (nz, no) = digit_counts(lines, starting_index);
    let digit = (chooser(nz, no) + 0x30) as u8;
    let new_lines = lines
        .iter()
        .filter(|p| digit_at_idx(*p, starting_index) == digit)
        .cloned()
        .collect::<Vec<String>>();

    value_reducer(&new_lines, starting_index + 1, chooser)
}

fn run_app() -> io::Result<()> {
    let mut lines = Vec::<String>::new();

    loop {
        let mut buffer = String::new();
        let bytes_read = io::stdin().read_line(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        lines.push(buffer.trim().to_string());
    }

    let gamma_rate = digit_filter(&lines, choose_greater);
    let epsilon_rate = digit_filter(&lines, choose_smaller);

    println!("Part 1: Power Consumption: {}", gamma_rate * epsilon_rate);

    let oxy_rating = value_reducer(&lines, 0, choose_greater);
    let co2_rating = value_reducer(&lines, 0, choose_smaller);

    println!("Part 2: Life support rating: {}", oxy_rating * co2_rating);

    Ok(())
}

fn main() {
    std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    static LINES: &[&str] = &[
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];

    #[test]
    fn sample_most() {
        assert_eq!(
            digit_filter(
                &LINES.iter().map(|x| x.to_string()).collect::<Vec<String>>(),
                choose_greater
            ),
            22
        );
    }
    #[test]
    fn sample_least() {
        assert_eq!(
            digit_filter(
                &LINES.iter().map(|x| x.to_string()).collect::<Vec<String>>(),
                choose_smaller
            ),
            9
        );
    }
    #[test]
    fn reducer_greater() {
        assert_eq!(
            value_reducer(
                &LINES.iter().map(|x| x.to_string()).collect::<Vec<String>>(),
                0,
                choose_greater
            ),
            23
        );
    }
    #[test]
    fn reducer_lesser() {
        assert_eq!(
            value_reducer(
                &LINES.iter().map(|x| x.to_string()).collect::<Vec<String>>(),
                0,
                choose_smaller
            ),
            10
        );
    }
}

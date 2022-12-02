use std::io;

fn fuel(loc: i64, spots: &[i64]) -> i64 {
    spots.iter().map(|&l| (l - loc).abs()).sum()
}

fn fuel2(loc: i64, spots: &[i64]) -> i64 {
    spots
        .iter()
        .map(|&l| {
            let n = (l - loc).abs();
            n * (n + 1) / 2
        })
        .sum()
}

fn find_minima(spots: &[i64], fuel_calc: fn(i64, &[i64]) -> i64) -> i64 {
    let left = *spots.iter().min().unwrap();
    let right = *spots.iter().max().unwrap();
    (left..=right).min_by_key(|&s| fuel_calc(s, spots)).unwrap()
}

fn main() -> io::Result<()> {
    let mut lines = Vec::<String>::new();

    loop {
        let mut buffer = String::new();
        let bytes_read = io::stdin().read_line(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        lines.push(buffer.trim().to_string());
    }

    let mut input = lines
        .iter()
        .map(|s| s.split(',').map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>()
        .concat();
    input.sort_unstable();

    let min_loc = input[input.len() >> 1];
    println!(
        "Part1: Minima at {}. Fuel to get there is {}.",
        min_loc,
        fuel(min_loc, &input)
    );

    let min_loc2 = find_minima(&input, fuel2);
    println!(
        "Part2: Minima at {}. Fuel to get there is {}.",
        min_loc2,
        fuel2(min_loc2, &input)
    );

    Ok(())
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//    use test_case::test_case;
//}

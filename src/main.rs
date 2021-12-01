use std::io;

fn part1(data: &Vec<i32>) -> Option<i32> {
    let first = *data.get(0)?;
    Some(
        data[1..]
            .iter()
            .fold((0, first), |accum, val| {
                let (counter, prev) = accum;
                (counter + (if *val > prev { 1 } else { 0 }), *val)
            })
            .0,
    )
}

fn part2(data: &Vec<i32>) -> Option<i32> {
    let first = *data.get(0)?;
    let second = *data.get(1)?;
    let third = *data.get(2)?;
    Some(
        data[3..]
            .iter()
            .fold((0, first + second + third, second, third), |accum, val| {
                let (counter, previous_sum, spot0, spot1) = accum;
                let new_sum = spot0 + spot1 + *val;
                (
                    counter + (if new_sum > previous_sum { 1 } else { 0 }),
                    new_sum,
                    spot1,
                    *val,
                )
            })
            .0,
    )
}

fn main() {
    let mut data: Vec<i32> = vec![];

    loop {
        let mut line = String::new();
        let bytes_read = io::stdin().read_line(&mut line).unwrap();
        if bytes_read == 0 {
            break;
        }
        match line.trim().parse::<i32>() {
            Ok(val) => {
                data.push(val);
            }
            Err(_) => {}
        }
    }

    let part1_result = part1(&data);
    let part2_result = part2(&data);

    println!("Day 1 results:");
    println!("Part 1: {}", part1_result.unwrap());
    println!("Part 2: {}", part2_result.unwrap());
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263] => Some(7); "sample")]
    #[test_case(&Vec::<i32>::new() => None; "no items")]
    #[test_case(&vec![1] => Some(0); "Just one item")]
    fn part1_sample(data: &Vec<i32>) -> Option<i32> {
        part1(data)
    }

    #[test_case(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263] => Some(5); "sample")]
    #[test_case(&Vec::<i32>::new() => None; "no items")]
    #[test_case(&vec![1] => None; "Just one item")]
    #[test_case(&vec![1, 2] => None; "two items")]
    #[test_case(&vec![1, 2, 3] => Some(0); "three items")]
    fn part2_sample(data: &Vec<i32>) -> Option<i32> {
        part2(data)
    }
}

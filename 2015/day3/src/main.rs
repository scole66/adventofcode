use ahash::AHashMap;
use lazy_static::lazy_static;
use regex::Regex;
use std::io;

#[derive(Debug, Default, PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn east(self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }
    fn west(self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }
    fn north(self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn south(self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }
}

fn input_valid(line: &str) -> bool {
    lazy_static! {
        static ref VALIDATE: Regex = Regex::new("^[<>^v]+$").unwrap();
    }
    VALIDATE.is_match(line)
}

fn part1(lines: &[String]) -> usize {
    let mut map: AHashMap<Position, i64> = AHashMap::default();
    let mut position = Position::default();

    *map.entry(position).or_insert(0) += 1;

    for line in lines.iter().filter(|s| input_valid(s)) {
        for ch in line.chars() {
            position = match ch {
                '^' => position.north(),
                'v' => position.south(),
                '>' => position.east(),
                _ => position.west(),
            };
            *map.entry(position).or_insert(0) += 1;
        }
    }

    map.len()
}

fn part2(lines: &[String]) -> usize {
    let mut map: AHashMap<Position, i64> = AHashMap::default();
    let mut santa_position = Position::default();
    let mut robot_position = Position::default();

    *map.entry(santa_position).or_insert(0) += 1;
    *map.entry(robot_position).or_insert(0) += 1;

    let spots = &mut [&mut santa_position, &mut robot_position];
    let mut spot_index = 0;

    for line in lines.iter().filter(|s| input_valid(s)) {
        for ch in line.chars() {
            let current = *spots[spot_index];
            let next_position = match ch {
                '^' => current.north(),
                'v' => current.south(),
                '>' => current.east(),
                _ => current.west(),
            };
            *map.entry(next_position).or_insert(0) += 1;
            *spots[spot_index] = next_position;
            spot_index = 1 - spot_index;
        }
    }

    map.len()
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

    println!("Part 1: Santa Visits {} homes", part1(&lines));
    println!("Part 2: Santa + Robot visit {} homes", part2(&lines));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(">" => 2)]
    #[test_case("^>v<" => 4)]
    #[test_case("^v^v^v^v^v" => 2)]
    fn part1_sample(line: &str) -> usize {
        part1(&[line.to_string()])
    }

    #[test_case("^v" => 3; "two instructions")]
    #[test_case("^>v<" => 3; "four instructions")]
    #[test_case("^v^v^v^v^v" => 11; "10 instructions")]
    fn part2_sample(line: &str) -> usize {
        part2(&[line.to_string()])
    }
}

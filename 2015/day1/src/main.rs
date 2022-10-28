use std::io;

fn part1(lines: &[String]) -> i32 {
    let mut floor = 0;
    for line in lines {
        floor += line.chars().fold(floor, |floor, ch| match ch {
            '(' => floor + 1,
            ')' => floor - 1,
            _ => floor,
        })
    }
    floor
}

fn part2(lines: &[String]) -> Option<i32> {
    let mut floor = 0;
    for line in lines {
        for (index, ch) in line.chars().enumerate() {
            floor += match ch {
                ')' => -1,
                '(' => 1,
                _ => 0,
            };
            if floor == -1 {
                return Some(index as i32 + 1);
            }
        }
    }
    None
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

    let part1_result = part1(&lines);
    println!("Part 1: Santa ends up on floor {part1_result}.");
    let part2_result = part2(&lines);
    println!("Part 2: Santa enters the basement at step {part2_result:?}.");

    Ok(())
}

fn main() {
    std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {err:?}");
            1
        }
    });
}

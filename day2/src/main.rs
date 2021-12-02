use regex::Regex;
use std::io;

static COMMAND_PATTERN: &str = r"^(?P<cmd>up|down|forward) +(?P<amount>\d+)\s*$";

#[derive(Debug, PartialEq, Eq)]
enum Command {
    Up,
    Down,
    Forward,
}

impl TryFrom<&str> for Command {
    type Error = &'static str;
    fn try_from(src: &str) -> Result<Self, Self::Error> {
        match src {
            "up" => Ok(Command::Up),
            "down" => Ok(Command::Down),
            "forward" => Ok(Command::Forward),
            _ => Err("badly formed command"),
        }
    }
}

fn act(accum: (i32, i32), action: &(Command, i32)) -> (i32, i32) {
    let (mut horiz, mut depth) = accum;
    let (cmd, amount) = action;

    match cmd {
        Command::Up => {
            depth -= amount;
        }
        Command::Down => {
            depth += amount;
        }
        Command::Forward => {
            horiz += amount;
        }
    }
    (horiz, depth)
}

fn act_with_aim(accum: (i32, i32, i32), action: &(Command, i32)) -> (i32, i32, i32) {
    let (mut horiz, mut depth, mut aim) = accum;
    let (cmd, amount) = action;

    match cmd {
        Command::Up => {
            aim -= amount;
        }
        Command::Down => {
            aim += amount;
        }
        Command::Forward => {
            horiz += amount;
            depth += aim * amount;
        }
    }

    (horiz, depth, aim)
}

fn main() {
    let mut data: Vec<(Command, i32)> = vec![];

    let matcher = Regex::new(COMMAND_PATTERN).unwrap();

    loop {
        let mut line = String::new();
        let bytes_read = io::stdin().read_line(&mut line).unwrap();
        if bytes_read == 0 {
            break;
        }
        if let Some(captures) = matcher.captures(line.trim()) {
            // will panic if the "amount" is greater than an i32
            data.push((
                Command::try_from(captures.name("cmd").unwrap().as_str()).unwrap(),
                captures
                    .name("amount")
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap(),
            ));
        }
    }

    let (h1, d1) = data.iter().fold((0, 0), act);
    let (h2, d2, aim) = data.iter().fold((0, 0, 0), act_with_aim);
    println!("Day 2 results:");

    println!("Part 1: H: {}, D: {}; result: {}", h1, d1, h1 * d1);
    println!(
        "Part 2: H: {}, D: {}, aim: {}, result: {}",
        h2,
        d2,
        aim,
        h2 * d2
    );
}

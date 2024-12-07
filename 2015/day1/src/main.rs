//! # Advent of Code 2015 - Day 1: Not Quite Lisp
//!
//! This module solves a puzzle where parentheses represent instructions to move
//! between floors. Opening parenthesis '(' means go up one floor, closing ')'
//! means go down one floor.
use std::io;

/// Calculates the final floor Santa ends up on after following all instructions.
/// Each '(' moves up one floor and each ')' moves down one floor.
///
/// # Arguments
///
/// * `lines` - A slice of strings containing parentheses instructions
///
/// # Returns
///
/// The final floor number Santa reaches
fn part1(lines: &[String]) -> i32 {
    let mut floor = 0;
    for line in lines {
        floor += line.chars().fold(floor, |floor, ch| match ch {
            '(' => floor + 1,
            ')' => floor - 1,
            _ => floor,
        });
    }
    floor
}

/// Finds the position of the first instruction that causes Santa to enter the basement
/// (floor -1).
///
/// # Arguments
///
/// * `lines` - A slice of strings containing parentheses instructions
///
/// # Returns
///
/// * `Some(i32)` - The 1-based position of the instruction that enters the basement
/// * `None` - If Santa never enters the basement
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
                return Some(i32::try_from(index).expect("index should be in range")  + 1);
            }
        }
    }
    None
}

/// Reads input from stdin and processes the puzzle solutions.
///
/// # Returns
///
/// * `Ok(())` - If input was successfully read and processed
/// * `Err(io::Error)` - If there was an error reading from stdin
///
/// # Errors
///
/// Will return an error if unable to read from stdin
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

    let start_time = std::time::Instant::now();
    let part1_result = part1(&lines);
    let part2_result = part2(&lines);
    let elapsed = start_time.elapsed();

    println!("Part 1: Santa ends up on floor {part1_result}.");
    println!("Part 2: Santa enters the basement at step {part2_result:?}.");
    println!("Time: {elapsed:?}");

    Ok(())
}

/// Main entry point for the program.
///
/// Executes the solution and handles any errors that occur.
/// Returns exit code 0 for success, 1 for error.
fn main() {
    std::process::exit(match run_app() {
        Ok(()) => 0,
        Err(err) => {
            eprintln!("error: {err:?}");
            1
        }
    });
}

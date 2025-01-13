use md5::{Digest, Md5};
use std::env;

fn six_zeroes(a: u8, b: u8, c: u8) -> bool {
    a == 0 && b == 0 && c == 0
}

fn five_zeroes(a: u8, b: u8, c: u8) -> bool {
    a == 0 && b == 0 && c & 0xF0 == 0
}

fn hashok(key: &str, number: i64, check: fn(u8, u8, u8) -> bool) -> bool {
    let input = format!("{key}{number}");
    let mut hasher = Md5::new();
    hasher.update(input);
    let result = hasher.finalize();
    check(result[0], result[1], result[2])
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let key = args.get(1).expect("argument needed");

    let mut goodval5 = None;
    let mut goodval6 = None;
    for attempt in 1.. {
        if goodval5.is_none() && hashok(key, attempt, five_zeroes) {
            goodval5 = Some(attempt);
        }
        if goodval6.is_none() && hashok(key, attempt, six_zeroes) {
            goodval6 = Some(attempt);
        }
        if goodval5.is_some() && goodval6.is_some() {
            break;
        }
    }
    println!("Part1: 5 leading zeroes with index {}", goodval5.unwrap());
    println!("Part2: 6 leading zeroes with index {}", goodval6.unwrap());
}

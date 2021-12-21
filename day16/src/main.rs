//! # Solution for Advent of Code 2021 Day 16
//!
//! Ref: [Advent of Code 2021 Day 16](https://adventofcode.com/2021/day/16)
//!

#![allow(dead_code, unused_variables, unused_imports)]

use ahash::AHashMap;
use anyhow::{self, Context};
use lazy_static::lazy_static;
use regex::Regex;
use std::io::{self, BufRead};
use std::str::Chars;

#[derive(Debug)]
struct BitStream {
    bits: Vec<u8>,
    current: usize,
}

fn char_to_bits(ch: char) -> [u8; 4] {
    match ch {
        '0' => [0, 0, 0, 0],
        '1' => [0, 0, 0, 1],
        '2' => [0, 0, 1, 0],
        '3' => [0, 0, 1, 1],
        '4' => [0, 1, 0, 0],
        '5' => [0, 1, 0, 1],
        '6' => [0, 1, 1, 0],
        '7' => [0, 1, 1, 1],
        '8' => [1, 0, 0, 0],
        '9' => [1, 0, 0, 1],
        'A' => [1, 0, 1, 0],
        'B' => [1, 0, 1, 1],
        'C' => [1, 1, 0, 0],
        'D' => [1, 1, 0, 1],
        'E' => [1, 1, 1, 0],
        _ => [1, 1, 1, 1],
    }
}

impl From<GoodString> for String {
    fn from(src: GoodString) -> Self {
        src.0
    }
}

impl From<GoodString> for BitStream {
    fn from(src: GoodString) -> Self {
        BitStream {
            bits: String::from(src)
                .chars()
                .map(char_to_bits)
                .flatten()
                .collect::<Vec<u8>>(),
            current: 0,
        }
    }
}
impl BitStream {
    fn bits(&mut self, count: usize) -> anyhow::Result<u64> {
        if self.current + count > self.bits.len() {
            anyhow::bail!("Not enough bits to satisfy request");
        }
        let result: u64 = self.bits[self.current..self.current + count]
            .iter()
            .fold(0_u64, |accum, &new| accum << 1 | new as u64);
        self.current += count;
        Ok(result)
    }
    //fn complete(&mut self) {
    //    self.current = (self.current + 3) & !3;
    //}
}
// impl<I: Iterator> Iterator for BitStream {
//     type Item = u8;
//
//     fn next(&mut self) -> Option<u8> {
//         todo!()
//     }
// }

#[derive(Debug)]
enum Packet {
    Literal { version: u8, value: u64 },
    Operator { version: u8, opcode: u8, sub_packets: Vec<Packet> },
}
impl Packet {
    fn version_sum(&self) -> u64 {
        match self {
            Packet::Literal { version, value: _ } => *version as u64,
            Packet::Operator { version, opcode: _, sub_packets } => {
                (*version as u64) + sub_packets.iter().map(|sp| sp.version_sum()).sum::<u64>()
            }
        }
    }

    fn evaluate(&self) -> u64 {
        match self {
            Packet::Literal { version: _, value } => *value,
            Packet::Operator { version: _, opcode, sub_packets } => match opcode {
                0 => sub_packets.iter().map(|p| p.evaluate()).sum::<u64>(),
                1 => sub_packets.iter().map(|p| p.evaluate()).product::<u64>(),
                2 => sub_packets.iter().map(|p| p.evaluate()).min().unwrap(),
                3 => sub_packets.iter().map(|p| p.evaluate()).max().unwrap(),
                5 => {
                    if sub_packets[0].evaluate() > sub_packets[1].evaluate() {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if sub_packets[0].evaluate() < sub_packets[1].evaluate() {
                        1
                    } else {
                        0
                    }
                }
                _ => {
                    if sub_packets[0].evaluate() == sub_packets[1].evaluate() {
                        1
                    } else {
                        0
                    }
                }
            },
        }
    }

    fn parse(src: &mut BitStream) -> anyhow::Result<(Packet, u64)> {
        let version = src.bits(3)? as u8;
        let id_code = src.bits(3)? as u8;
        let mut consumed = 6;
        match id_code {
            4 => {
                // Literal
                let mut value: u64 = 0;
                loop {
                    let continuation = src.bits(1)?;
                    let partial = src.bits(4)?;
                    consumed += 5;
                    if value & 0xF000_0000_0000_0000 != 0 {
                        anyhow::bail!("Overflow in literal creation");
                    }
                    value = value << 4 | partial;
                    if continuation == 0 {
                        break;
                    }
                }
                Ok((Packet::Literal { version, value }, consumed))
            }
            _ => {
                // operator
                let length_type_id = src.bits(1)?;
                consumed += 1;
                match length_type_id {
                    0 => {
                        let total_length_in_bits = src.bits(15)?;
                        consumed += 15;
                        let mut sub_bits_consumed = 0;
                        let mut sub_packets: Vec<Packet> = Vec::new();
                        while sub_bits_consumed < total_length_in_bits {
                            let (pkt, sub_bits) = Packet::parse(src)?;
                            sub_packets.push(pkt);
                            sub_bits_consumed += sub_bits;
                            if sub_bits_consumed > total_length_in_bits {
                                anyhow::bail!("Sub packet took more bits than expected");
                            }
                        }
                        consumed += sub_bits_consumed;
                        Ok((Packet::Operator { version, opcode: id_code, sub_packets }, consumed))
                    }
                    _ => {
                        let number_of_sub_packets = src.bits(11)?;
                        consumed += 11;
                        let mut sub_packets: Vec<Packet> = Vec::new();
                        for _ in 0..number_of_sub_packets {
                            let (pkt, sub_bits) = Packet::parse(src)?;
                            consumed += sub_bits;
                            sub_packets.push(pkt);
                        }
                        Ok((Packet::Operator { version, opcode: id_code, sub_packets }, consumed))
                    }
                }
            }
        }
    }
}
impl TryFrom<GoodString> for Packet {
    type Error = anyhow::Error;
    fn try_from(src: GoodString) -> Result<Self, Self::Error> {
        let bs = BitStream::from(src);
        Packet::try_from(bs)
    }
}
impl TryFrom<BitStream> for Packet {
    type Error = anyhow::Error;
    fn try_from(mut src: BitStream) -> Result<Self, Self::Error> {
        Packet::parse(&mut src).map(|(p, _)| p)
    }
}

fn version_sum(src: GoodString) -> anyhow::Result<u64> {
    let packet_tree = Packet::try_from(src)?;
    Ok(packet_tree.version_sum())
}

fn evaluate(src: GoodString) -> anyhow::Result<u64> {
    let packet_tree = Packet::try_from(src)?;
    Ok(packet_tree.evaluate())
}

// NewType meaning: a String that has only valid characters.
#[derive(Debug, Clone)]
struct GoodString(String);

fn validate(src: String) -> anyhow::Result<GoodString> {
    lazy_static! {
        static ref VALID_PATTERN: Regex = Regex::new("^[0-9A-F]+$").unwrap();
    }
    if !VALID_PATTERN.is_match(&src) {
        anyhow::bail!("Invalid characters in input pattern");
    }
    Ok(GoodString(src))
}

fn main() -> Result<(), anyhow::Error> {
    let stdin = io::stdin();

    let input = stdin
        .lock()
        .lines()
        .map(|r| r.map_err(anyhow::Error::from))
        .map(|r| r.and_then(validate))
        .collect::<anyhow::Result<Vec<_>>>()
        .context("Failed to parse puzzle input from stdin")?;

    let first_line = input.first().ok_or(anyhow::anyhow!("Need one line"))?;

    println!("Part 1: Sum of versions: {}", version_sum(first_line.clone())?);

    println!("Part 2: Evaluate: {}", evaluate(first_line.clone())?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("D2FE28" => 6)]
    #[test_case("38006F45291200" => 9)]
    #[test_case("8A004A801A8002F478" => 16)]
    #[test_case("620080001611562C8802118E34" => 12)]
    #[test_case("C0015000016115A2E0802F182340" => 23)]
    #[test_case("A0016C880162017C3686B18A3D4780" => 31)]
    fn version_sum(src: &str) -> u64 {
        let s = validate(src.to_string()).unwrap();
        super::version_sum(s).unwrap()
    }

    #[test_case("C200B40A82" => 3)]
    #[test_case("04005AC33890" => 54)]
    #[test_case("880086C3E88112" => 7)]
    #[test_case("CE00C43D881120" => 9)]
    #[test_case("D8005AC2A8F0" => 1)]
    #[test_case("F600BC2D8F" => 0)]
    #[test_case("9C005AC2F8F0" => 0)]
    #[test_case("9C0141080250320F1802104A08" => 1)]
    fn evaluate(src: &str) -> u64 {
        let s = validate(src.to_string()).unwrap();
        super::evaluate(s).unwrap()
    }
}

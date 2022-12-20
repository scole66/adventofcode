//! # Solution for Advent of Code 2022 Day 20: Grove Positioning System
//!
//! Ref: [Advent of Code 2022 Day 20](https://adventofcode.com/2022/day/20)
//!
use std::cmp::Ordering;
use std::io::{self, Read};
use std::iter::Iterator;
use std::str::FromStr;

const DECRYPTION_KEY: i64 = 811589153;

struct Input(Vec<i32>);
impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s
            .lines()
            .map(|l| l.parse::<i32>().map_err(anyhow::Error::from))
            .collect::<anyhow::Result<Vec<i32>>>()?;
        if v.len() > i32::MAX as usize {
            anyhow::bail!("too many values")
        }
        if !v.contains(&0) {
            anyhow::bail!("must have an entry with value 0")
        }
        Ok(Input(v))
    }
}

enum CoordStyle {
    PlainText,
    Decrypted,
}
impl Input {
    fn mix(&self) -> Vec<i64> {
        let mut mixed = self.0.iter().map(|&v| v as i64).enumerate().collect::<Vec<_>>();
        Self::inner_mix(&mut mixed, self.0.len());
        mixed.into_iter().map(|(_, val)| val).collect()
    }

    fn inner_mix(mixed: &mut [(usize, i64)], len: usize) {
        for spot in 0..len {
            let pos = mixed
                .iter()
                .position(|&(initial_spot, _)| initial_spot == spot)
                .unwrap();
            let target = mixed[pos].1;
            let new_pos = (pos as isize + target as isize).rem_euclid(len as isize - 1) as usize;
            match new_pos.cmp(&pos) {
                Ordering::Less => {
                    mixed[new_pos..=pos].rotate_right(1);
                }
                Ordering::Equal => {}
                Ordering::Greater => {
                    mixed[pos..=new_pos].rotate_left(1);
                }
            }
        }
    }

    fn coords(&self, style: CoordStyle) -> (i64, i64, i64) {
        let mixed = match style {
            CoordStyle::PlainText => self.mix(),
            CoordStyle::Decrypted => self.mix2(),
        };

        let position_of_zero = mixed.iter().position(|&v| v == 0).unwrap();
        (
            mixed[(1000 + position_of_zero) % mixed.len()],
            mixed[(2000 + position_of_zero) % mixed.len()],
            mixed[(3000 + position_of_zero) % mixed.len()],
        )
    }

    fn score(&self, style: CoordStyle) -> isize {
        let coords = self.coords(style);
        let x = coords.0 as isize;
        let y = coords.1 as isize;
        let z = coords.2 as isize;
        x + y + z
    }

    fn mix2(&self) -> Vec<i64> {
        let mut mixed = self
            .0
            .iter()
            .map(|val| *val as i64 * DECRYPTION_KEY)
            .enumerate()
            .collect::<Vec<_>>();
        for _ in 0..10 {
            Self::inner_mix(&mut mixed, self.0.len());
        }
        mixed.into_iter().map(|(_, val)| val).collect()
    }
}

fn part1(input: &str) -> anyhow::Result<isize> {
    let encoded = input.parse::<Input>()?;
    Ok(encoded.score(CoordStyle::PlainText))
}

fn part2(input: &str) -> anyhow::Result<isize> {
    let encoded = input.parse::<Input>()?;
    Ok(encoded.score(CoordStyle::Decrypted))
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;

    println!("Part1: {}", part1(&input)?);
    println!("Part2: {}", part2(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    static SAMPLE: &str = indoc::indoc! {"
        1
        2
        -3
        3
        -2
        0
        4
    "};

    #[test]
    fn part1_sample() {
        assert_eq!(part1(SAMPLE).unwrap(), 3);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(SAMPLE).unwrap(), 1623178306);
    }

    #[test_case(&[0,0,1,0,0] => [0,0,0,0,1].to_vec(); "move one to the right")]
    #[test_case(&[0,0,-1,0,0] => [0,0,-1,0,0].to_vec(); "move one to the left")]
    #[test_case(&[0,0,4,0,0] => [0,0,0,4,0].to_vec(); "move right, wrapping")]
    #[test_case(&[0,0,-4,0,0] => [0,0,0,-4,0].to_vec(); "move left, wrapping")]
    #[test_case(&[1,2,-3,3,-2,0,4] => [-2, 1, 2, -3, 4, 0, 3].to_vec(); "from the sample")]
    fn mix(input: &[i32]) -> Vec<i64> {
        let inp = Input(input.to_vec());
        inp.mix()
    }
}

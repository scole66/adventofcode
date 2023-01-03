//! # Solution for Advent of Code 2015 Day 15: Science for Hungry People
//!
//! Ref: [Advent of Code 2015 Day 15](https://adventofcode.com/2015/day/15)
//!
use ahash::AHashMap;
use anyhow::{anyhow, Error, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use std::io::{self, Read};
use std::str::FromStr;

struct Properties {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

struct Datum {
    name: String,
    properties: Properties,
}
impl FromStr for Datum {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static PATTERN: Lazy<Regex> = Lazy::new(|| {
            Regex::new(
            r"^(?P<name>[a-zA-Z]+): capacity (?P<capacity>0|-?[1-9][0-9]*), durability (?P<durability>0|-?[1-9][0-9]*), flavor (?P<flavor>0|-?[1-9][0-9]*), texture (?P<texture>0|-?[1-9][0-9]*), calories (?P<calories>0|-?[1-9][0-9]*)$"
        ).unwrap()
        });

        let caps = PATTERN.captures(s).ok_or_else(|| anyhow!("bad input line: {s}"))?;
        let name = caps["name"].to_string();
        let capacity = caps["capacity"].parse()?;
        let durability = caps["durability"].parse()?;
        let flavor = caps["flavor"].parse()?;
        let texture = caps["texture"].parse()?;
        let calories = caps["calories"].parse()?;

        Ok(Datum { name, properties: Properties { capacity, durability, flavor, texture, calories } })
    }
}

struct Details {
    ingredients: AHashMap<String, Properties>,
}
impl FromStr for Details {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Details {
            ingredients: s
                .lines()
                .map(|line| {
                    line.parse::<Datum>()
                        .map(|Datum { name, properties }| (name, properties))
                })
                .collect::<Result<AHashMap<_, _>>>()?,
        })
    }
}

impl Details {
    fn cookie_score(&self, cookie: &Cookie) -> i64 {
        let capacity = cookie
            .ingredients
            .iter()
            .map(|(name, amt)| self.ingredients[name].capacity * amt)
            .sum::<i64>()
            .max(0);
        let durability = cookie
            .ingredients
            .iter()
            .map(|(name, amt)| self.ingredients[name].durability * amt)
            .sum::<i64>()
            .max(0);
        let flavor = cookie
            .ingredients
            .iter()
            .map(|(name, amt)| self.ingredients[name].flavor * amt)
            .sum::<i64>()
            .max(0);
        let texture = cookie
            .ingredients
            .iter()
            .map(|(name, amt)| self.ingredients[name].texture * amt)
            .sum::<i64>()
            .max(0);

        capacity * durability * flavor * texture
    }

    fn cookie_calories(&self, cookie: &Cookie) -> i64 {
        cookie
            .ingredients
            .iter()
            .map(|(name, amt)| self.ingredients[name].calories * amt)
            .sum()
    }
}

struct Cookie {
    ingredients: AHashMap<String, i64>,
}
impl Cookie {
    fn new(ingredient_amounts: &[i32], details: &Details) -> Self {
        assert_eq!(ingredient_amounts.len(), details.ingredients.len());
        let cookie = details
            .ingredients
            .keys()
            .zip(ingredient_amounts)
            .map(|(a, b)| (a.clone(), i64::from(*b)))
            .collect::<AHashMap<_, _>>();
        Cookie { ingredients: cookie }
    }
}

fn ingredient_combinations(num_ingredients: i32, num_teaspoons: i32) -> Vec<Vec<i32>> {
    fn inner(partial: &[i32], num: i32, left: i32) -> Vec<Vec<i32>> {
        assert!(num >= 2);
        if num == 2 {
            (0..=left)
                .map(|us| {
                    let mut result = Vec::with_capacity(partial.len() + 2);
                    result.extend_from_slice(partial);
                    result.push(us);
                    result.push(left - us);
                    result
                })
                .collect::<Vec<_>>()
        } else {
            (0..=left)
                .flat_map(|us| {
                    let mut header = Vec::with_capacity(partial.len() + 1);
                    header.extend_from_slice(partial);
                    header.push(us);
                    inner(&header, num - 1, left - us)
                })
                .collect::<Vec<_>>()
        }
    }

    inner(&[], num_ingredients, num_teaspoons)
}

fn part1(input: &str) -> Result<i64> {
    let ingredient_details = input.parse::<Details>()?;
    let num_ingredients = i32::try_from(ingredient_details.ingredients.len())?;
    let best_score = ingredient_combinations(num_ingredients, 100)
        .into_iter()
        .map(|v| Cookie::new(&v, &ingredient_details))
        .map(|cookie| ingredient_details.cookie_score(&cookie))
        .max()
        .expect("should be data");
    Ok(best_score)
}

fn part2(input: &str) -> Result<i64> {
    let ingredient_details = input.parse::<Details>()?;
    let num_ingredients = i32::try_from(ingredient_details.ingredients.len())?;
    let best_score = ingredient_combinations(num_ingredients, 100)
        .into_iter()
        .map(|v| Cookie::new(&v, &ingredient_details))
        .filter(|cookie| ingredient_details.cookie_calories(cookie) == 500)
        .map(|cookie| ingredient_details.cookie_score(&cookie))
        .max()
        .expect("should be data");
    Ok(best_score)
}

fn main() -> Result<()> {
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

    static SAMPLE: &str = indoc::indoc! {"
        Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
        Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
    "};

    #[test]
    fn part1_sample() {
        assert_eq!(part1(SAMPLE).unwrap(), 62842880);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(SAMPLE).unwrap(), 57600000);
    }
}

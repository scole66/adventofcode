//! # Solution for Advent of Code 2021 Day 6
//!
//! Ref: [Advent of Code 2021 Day 6](https://adventofcode.com/2021/day/6)

use std::io;

/// Our school of fish is represented as counts of fish in each stage of the reproduction state machine
#[derive(Debug, Default)]
struct School {
    num_fish_at_stage: [usize; 9],
}

impl School {
    /// Create a school of fish from lines of input
    fn new(lines: &[String]) -> Self {
        let mut school = Self::default();
        for line in lines {
            line.split(',')
                .filter_map(|item| item.parse::<u8>().ok())
                .filter(|num| *num <= 8)
                .for_each(|num| school.num_fish_at_stage[num as usize] += 1)
        }

        school
    }

    /// The number of fish in the school
    fn population(&self) -> usize {
        self.num_fish_at_stage.iter().sum()
    }

    /// Run the school through one generation of life
    fn generation(&mut self) {
        let mut next_generation = School::default();
        for stage in 0..=7 {
            // Most stages just move on to the next stage
            next_generation.num_fish_at_stage[stage] = self.num_fish_at_stage[stage + 1];
        }
        // But stage zero moves to stage six, as well as all the ones from stage 7.
        next_generation.num_fish_at_stage[6] += self.num_fish_at_stage[0];
        // And also, new fish are spawned in stage eight, equal to the number of fish in (previous) stage 0
        next_generation.num_fish_at_stage[8] = self.num_fish_at_stage[0];

        // Now copy back, replacing the previous generation with this new generation.
        for idx in 0..=8 {
            self.num_fish_at_stage[idx] = next_generation.num_fish_at_stage[idx];
        }
    }
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

    let mut school = School::new(&lines);
    for _ in 0..80 {
        school.generation();
    }
    println!("Part1: After 80 generations, there are {} fish", school.population());
    for _ in 80..256 {
        school.generation();
    }
    println!("Part2: After 256 generations, there are {} fish", school.population());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(18 => 26)]
    #[test_case(80 => 5934)]
    #[test_case(256 => 26984457539)]
    fn generations(num_gen: usize) -> usize {
        let lines = &["3,4,3,1,2".to_string()];
        let mut school = School::new(lines);
        for _ in 0..num_gen {
            school.generation();
        }
        school.population()
    }
}

//! # Solution for Advent of Code 2024 Day 9:
//!
//! Ref: [Advent of Code 2024 Day 9](https://adventofcode.com/2024/day/9)
//!
use ahash::AHashMap;
use anyhow::{anyhow, Error, Result};
use std::io::{self, Read};
use std::str::FromStr;

/// Represents the raw disk map input as a sequence of digits
struct DiskMap {
    map: Vec<u32>,
}

impl FromStr for DiskMap {
    type Err = Error;

    /// Parses a string of digits into a `DiskMap`
    ///
    /// # Arguments
    ///
    /// * `s` - Input string containing digits representing file sizes and gaps
    ///
    /// # Returns
    ///
    /// * `Ok(DiskMap)` - Successfully parsed disk map
    /// * `Err` - If input contains invalid digits
    fn from_str(s: &str) -> Result<Self> {
        Ok(Self {
            map: s
                .trim()
                .chars()
                .map(|ch| {
                    ch.to_digit(10)
                        .ok_or_else(|| anyhow!("Improper digit in disk map '{ch}'"))
                })
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

/// Represents the content of a block on the disk - either empty or containing a file
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum BlockContent {
    /// An empty block that can be used for file storage
    Empty,
    /// An empty block that can be used for file storage
    File(usize),
}

/// Display implementation for `BlockContent`
impl std::fmt::Display for BlockContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockContent::Empty => write!(f, "."),
            BlockContent::File(id) => write!(f, "{}", id % 10),
        }
    }
}

/// Represents the expanded disk map with file locations and metadata
#[derive(Debug)]
struct ExpandedDiskMap {
    /// Vector of blocks representing the disk contents
    map: Vec<BlockContent>,
    /// Highest file ID in use
    max_id: usize,
    /// Map of file IDs to their location and size
    file_data: AHashMap<usize, (usize, usize)>,
}

impl From<&DiskMap> for ExpandedDiskMap {
    /// Converts a raw `DiskMap` into an expanded representation
    ///
    /// Processes alternating sequences of file blocks and empty blocks,
    /// assigning file IDs and tracking their locations
    fn from(value: &DiskMap) -> Self {
        let mut fileid = 0;
        let mut map = Vec::new();
        let mut iter = value.map.iter();
        let mut file_data = AHashMap::new();
        while let Some(&file_count) = iter.next() {
            file_data.insert(fileid, (map.len(), file_count as usize));
            for _ in 0..file_count {
                map.push(BlockContent::File(fileid));
            }
            fileid += 1;
            if let Some(&empty_count) = iter.next() {
                for _ in 0..empty_count {
                    map.push(BlockContent::Empty);
                }
            }
        }
        ExpandedDiskMap {
            map,
            max_id: fileid - 1,
            file_data,
        }
    }
}

impl ExpandedDiskMap {
    /// Performs basic defragmentation by moving files toward the beginning
    fn compact(&mut self) {
        let mut write_idx = 0;
        // find the first empty spot to write to
        while self.map[write_idx] != BlockContent::Empty {
            write_idx += 1;
        }
        let mut read_idx = self.map.len() - 1;
        // find the last nonempty spot to read from
        while self.map[read_idx] == BlockContent::Empty {
            read_idx -= 1;
        }
        while write_idx < read_idx {
            self.map.swap(read_idx, write_idx);
            read_idx -= 1;
            while self.map[read_idx] == BlockContent::Empty {
                read_idx -= 1;
            }
            write_idx += 1;
            while self.map[write_idx] != BlockContent::Empty {
                write_idx += 1;
            }
        }
    }

    /// Calculates the checksum of the current disk state
    ///
    /// The checksum is the sum of (`block_index` * `file_id`) for all file blocks
    fn checksum(&self) -> usize {
        self.map
            .iter()
            .enumerate()
            .map(|(index, element)| match element {
                BlockContent::Empty => 0,
                BlockContent::File(id) => id.checked_mul(index).unwrap(),
            })
            .sum()
    }

    /// Performs defragmentation while maintaining file contiguity
    ///
    /// Moves files to minimize the checksum while ensuring each file's blocks
    /// remain together
    fn compact_nofrag(&mut self) {
        let mut filenum = self.max_id;
        while filenum > 0 {
            let (src, bytes_to_find) = self.file_data.get(&filenum).unwrap();
            let mut dest = 0;
            while dest < *src {
                while self.map[dest] != BlockContent::Empty {
                    dest += 1;
                }
                let mut empty_after = dest + 1;
                while self.map[empty_after] == BlockContent::Empty {
                    empty_after += 1;
                }
                let empty_size = empty_after - dest;
                if empty_size >= *bytes_to_find && dest < *src {
                    for idx in 0..*bytes_to_find {
                        self.map.swap(dest + idx, *src + idx);
                    }
                    break;
                }
                dest = empty_after;
            }
            filenum -= 1;
        }
    }
}

/// Solves part 1: basic defragmentation
fn part1(input: &DiskMap) -> usize {
    let mut map = ExpandedDiskMap::from(input);
    map.compact();
    map.checksum()
}

/// Solves part 2: defragmentation with contiguity constraints
fn part2(input: &DiskMap) -> usize {
    let mut map = ExpandedDiskMap::from(input);
    map.compact_nofrag();
    map.checksum()
}

/// Main function that reads input and solves both parts
///
/// # Errors
///
/// Returns an error if:
/// * Failed to read from stdin
/// * Failed to parse the input
fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;
    let input = input.parse::<DiskMap>()?;

    let start_time = std::time::Instant::now();
    let part1 = part1(&input);
    let part2 = part2(&input);
    let elapsed = start_time.elapsed();

    println!("Part1: {part1}");
    println!("Part2: {part2}");
    println!("Time: {elapsed:?}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = indoc::indoc! {"
        2333133121414131402
    "};

    #[test]
    fn part1_sample() {
        assert_eq!(part1(&SAMPLE.parse::<DiskMap>().unwrap()), 1928);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(&SAMPLE.parse::<DiskMap>().unwrap()), 2858);
    }
}

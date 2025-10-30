//! # Solution for Advent of Code 2022 Day 7: No Space Left On Device
//!
//! Ref: [Advent of Code 2022 Day 7](https://adventofcode.com/2022/day/7)
//!
use ahash::AHashMap;
use std::io::{self, Read};
use std::iter::{Iterator, Peekable};

struct Input(Vec<String>);

impl From<&str> for Input {
    fn from(s: &str) -> Self {
        Input(s.lines().map(String::from).collect())
    }
}
impl From<String> for Input {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

#[derive(Debug)]
enum DirectoryEntry {
    File(usize),
    Directory(Box<Directory>),
}

#[derive(Debug)]
struct Directory(AHashMap<String, DirectoryEntry>);

impl TryFrom<Input> for Directory {
    type Error = anyhow::Error;
    fn try_from(input: Input) -> Result<Self, Self::Error> {
        let mut iter = input.0.iter().cloned().peekable();

        // So: we always start with a $ cd /
        let first_line = iter.next().unwrap_or_default();
        if first_line != "$ cd /" {
            anyhow::bail!("Bad first line (expected `$ cd /'; saw {first_line:?}");
        }

        // Now set up the top.
        let mut top = Directory(AHashMap::new());
        dirparse(&mut iter, &mut top)?;

        Ok(top)
    }
}

fn dirparse(cmds: &mut Peekable<impl Iterator<Item = String>>, curdir: &mut Directory) -> anyhow::Result<()> {
    loop {
        let line = cmds.next();
        match line {
            None => return Ok(()),
            Some(line) => {
                if line == "$ cd .." {
                    return Ok(());
                }
                if line == "$ ls" {
                    loop {
                        let entry = cmds.peek();
                        if let Some(entry) = entry {
                            if entry.starts_with('$') {
                                break;
                            }
                        } else {
                            break;
                        }
                        let entry = cmds.next().expect("Valid due to prior check");
                        let file_metadata = entry.split(' ').collect::<Vec<_>>();
                        if file_metadata.len() != 2 {
                            anyhow::bail!("Bad directory entry")
                        }
                        if file_metadata[0] == "dir" {
                            curdir.0.insert(
                                file_metadata[1].to_string(),
                                DirectoryEntry::Directory(Box::new(Directory(AHashMap::new()))),
                            );
                        } else {
                            let file_size = file_metadata[0].parse::<usize>()?;
                            curdir
                                .0
                                .insert(file_metadata[1].to_string(), DirectoryEntry::File(file_size));
                        }
                    }
                    continue;
                }
                if line.starts_with("$ cd ") {
                    let words = line.split(' ').collect::<Vec<_>>();
                    let dest_dir = words[words.len() - 1];
                    let entry = curdir
                        .0
                        .get_mut(dest_dir)
                        .ok_or_else(|| anyhow::anyhow!("undefined directory"))?;
                    if let DirectoryEntry::Directory(dir) = entry {
                        dirparse(cmds, dir)?;
                    } else {
                        anyhow::bail!("{dest_dir}: not a directory");
                    }
                    continue;
                }
                anyhow::bail!("Invalid terminal command");
            }
        }
    }
}

impl Directory {
    fn size(&self) -> usize {
        self.0.values().map(|entry| entry.size()).sum()
    }
}

impl DirectoryEntry {
    fn size(&self) -> usize {
        match self {
            DirectoryEntry::File(size) => *size,
            DirectoryEntry::Directory(dir) => dir.size(),
        }
    }
}

fn sum_sizes_less(input: &Directory) -> usize {
    // sum directories with small size
    let children_size_sum = input
        .0
        .iter()
        .map(|de| {
            if let DirectoryEntry::Directory(dir) = de.1 {
                sum_sizes_less(dir)
            } else {
                0
            }
        })
        .sum();
    let self_size = input.size();
    if self_size <= 100000 {
        children_size_sum + self_size
    } else {
        children_size_sum
    }
}

fn part1(input: &Directory) -> usize {
    sum_sizes_less(input)
}

impl From<&Directory> for Vec<usize> {
    fn from(dir: &Directory) -> Self {
        let mut result = vec![];
        result.push(dir.size());
        for (_, item) in dir.0.iter() {
            if let DirectoryEntry::Directory(child) = item {
                let child_sizes: Vec<usize> = (&**child).into();
                result.extend(child_sizes);
            }
        }

        result
    }
}

fn part2(input: &Directory) -> usize {
    let target_size = 30000000 - (70000000 - input.size());
    let all_sizes: Vec<usize> = input.into();
    let mut big_sizes: Vec<usize> = all_sizes.into_iter().filter(|&size| size >= target_size).collect();
    big_sizes.sort();
    big_sizes[0]
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;
    let lines = Input::from(input);

    let directory = Directory::try_from(lines)?;

    println!("Part1: {}", part1(&directory));
    println!("Part2: {}", part2(&directory));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = indoc::indoc! {"
        $ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k
    "};

    #[test]
    fn part1_sample() {
        let input = Directory::try_from(Input::from(SAMPLE)).unwrap();
        assert_eq!(part1(&input), 95437);
    }

    #[test]
    fn part2_sample() {
        let input = Directory::try_from(Input::from(SAMPLE)).unwrap();
        assert_eq!(part2(&input), 24933642);
    }
}

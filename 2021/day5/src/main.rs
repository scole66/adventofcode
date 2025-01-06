use ahash::AHashMap;
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt::{self, Display};
use std::io;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    row: i64,
    col: i64,
}

#[derive(Debug, Default)]
struct SeaFloor {
    grid: AHashMap<Position, i64>,
}

impl Display for SeaFloor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.edges() {
            None => f.write_str("empty"),
            Some((left, right, top, bottom)) => {
                f.write_str("\n")?;
                for row in top..bottom + 1 {
                    let x = (left..right + 1)
                        .map(|col| {
                            let pos = Position { row, col };
                            let hv = *self.grid.get(&pos).unwrap_or(&0);
                            match hv {
                                v if v <= 0 => '.',
                                v if (1..10).contains(&v) => (0x30 + v as u8) as char,
                                _ => '!',
                            }
                        })
                        .collect::<String>();
                    writeln!(f, "{x}")?;
                }
                Ok(())
            }
        }
    }
}

impl SeaFloor {
    fn add_vent(&mut self, vent: &VentDescription, diagonals_ok: bool) {
        Walker::new(vent, diagonals_ok).for_each(|pos| *self.grid.entry(pos).or_insert(0) += 1);
    }

    fn construct(lines: &[String], diagonals_ok: bool) -> Self {
        let vents: Vec<VentDescription> = lines.iter().filter_map(|l| parse_line(l)).collect();

        let mut seabed = Self::default();
        for vent in vents.iter() {
            seabed.add_vent(vent, diagonals_ok);
        }

        seabed
    }

    fn hazardous_location_count(&self) -> usize {
        self.grid.iter().filter(|(_, val)| **val >= 2).count()
    }

    fn edges(&self) -> Option<(i64, i64, i64, i64)> {
        // Determines the horiz and vert extents of the seafloor. An empty seafloor has no extents (and returns None).
        let mut iter = self.grid.iter();
        match iter.next() {
            None => None,
            Some((pos, _)) => {
                let initial_value = (pos.col, pos.col, pos.row, pos.row);
                Some(iter.fold(initial_value, |(left, right, top, bottom), (pos, _)| {
                    (
                        if pos.col < left { pos.col } else { left },
                        if pos.col > right { pos.col } else { right },
                        if pos.row < top { pos.row } else { top },
                        if pos.row > bottom { pos.row } else { bottom },
                    )
                }))
            }
        }
    }
}

// Walker: This is an iterator definition for something that walks the seabed, returning Positions based on a particular
// vent definition.
#[derive(Debug)]
struct Walker {
    pos: Position,
    dx: i64,
    dy: i64,
    remaining: i64,
}

impl Iterator for Walker {
    type Item = Position;
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            None
        } else {
            let result = self.pos;
            self.pos = Position {
                row: result.row + self.dy,
                col: result.col + self.dx,
            };
            self.remaining -= 1;
            Some(result)
        }
    }
}

impl Walker {
    fn new(vent: &VentDescription, diagonals_ok: bool) -> Self {
        match vent {
            VentDescription::Horiz {
                row_value,
                col_start,
                col_end,
            } => Self {
                pos: Position {
                    row: *row_value,
                    col: *col_start,
                },
                dx: 1,
                dy: 0,
                remaining: *col_end - *col_start + 1,
            },
            VentDescription::Vert {
                col_value,
                row_start,
                row_end,
            } => Self {
                pos: Position {
                    row: *row_start,
                    col: *col_value,
                },
                dx: 0,
                dy: 1,
                remaining: *row_end - *row_start + 1,
            },
            VentDescription::UpAndRight {
                col_start,
                row_start,
                length,
            } => Self {
                pos: Position {
                    row: *row_start,
                    col: *col_start,
                },
                dx: 1,
                dy: -1,
                remaining: if diagonals_ok { *length } else { 0 },
            },
            VentDescription::DownAndRight {
                col_start,
                row_start,
                length,
            } => Self {
                pos: Position {
                    row: *row_start,
                    col: *col_start,
                },
                dx: 1,
                dy: 1,
                remaining: if diagonals_ok { *length } else { 0 },
            },
        }
    }
}

#[derive(Debug)]
enum VentDescription {
    Horiz {
        row_value: i64,
        col_start: i64,
        col_end: i64,
    },
    Vert {
        col_value: i64,
        row_start: i64,
        row_end: i64,
    },
    UpAndRight {
        col_start: i64,
        row_start: i64,
        length: i64,
    },
    DownAndRight {
        col_start: i64,
        row_start: i64,
        length: i64,
    },
}

fn parse_line(line: &str) -> Option<VentDescription> {
    lazy_static! {
        static ref PARSE: Regex =
            Regex::new("^(?P<x1>[0-9]+),(?P<y1>[0-9]+) -> (?P<x2>[0-9]+),(?P<y2>[0-9]+)$").unwrap();
    }
    PARSE.captures(line).and_then(|captures| {
        let x1 = captures.name("x1").unwrap().as_str().parse::<i64>().unwrap();
        let y1 = captures.name("y1").unwrap().as_str().parse::<i64>().unwrap();
        let x2 = captures.name("x2").unwrap().as_str().parse::<i64>().unwrap();
        let y2 = captures.name("y2").unwrap().as_str().parse::<i64>().unwrap();
        if x1 == x2 {
            let (top, bottom) = if y1 > y2 { (y2, y1) } else { (y1, y2) };
            Some(VentDescription::Vert {
                col_value: x1,
                row_start: top,
                row_end: bottom,
            })
        } else if y1 == y2 {
            let (left, right) = if x1 > x2 { (x2, x1) } else { (x1, x2) };
            Some(VentDescription::Horiz {
                row_value: y1,
                col_start: left,
                col_end: right,
            })
        } else if (y1 - y2).abs() == (x1 - x2).abs() {
            let (top, bottom) = if y1 > y2 { (y2, y1) } else { (y1, y2) };
            let left = if x1 > x2 { x2 } else { x1 };
            if (y2 - y1).signum() != (x2 - x1).signum() {
                Some(VentDescription::UpAndRight {
                    col_start: left,
                    row_start: bottom,
                    length: (x1 - x2).abs() + 1,
                })
            } else {
                Some(VentDescription::DownAndRight {
                    col_start: left,
                    row_start: top,
                    length: (x1 - x2).abs() + 1,
                })
            }
        } else {
            None
        }
    })
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

    let sf_part1 = SeaFloor::construct(&lines, false);
    println!("Part 1: {} hazardous locations.", sf_part1.hazardous_location_count());

    let sf_part2 = SeaFloor::construct(&lines, true);
    println!("Part 2: {} hazardous locations.", sf_part2.hazardous_location_count());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_LINES: &[&str] = &[
        "0,9 -> 5,9",
        "8,0 -> 0,8",
        "9,4 -> 3,4",
        "2,2 -> 2,1",
        "7,0 -> 7,4",
        "6,4 -> 2,0",
        "0,9 -> 2,9",
        "3,4 -> 1,4",
        "0,0 -> 8,8",
        "5,5 -> 8,2",
    ];

    #[test]
    fn sample_part1() {
        let sf = SeaFloor::construct(
            &TEST_LINES.iter().map(|s| s.to_string()).collect::<Vec<String>>(),
            false,
        );
        assert_eq!(sf.hazardous_location_count(), 5);
    }
    #[test]
    fn sample_part2() {
        let sf = SeaFloor::construct(&TEST_LINES.iter().map(|s| s.to_string()).collect::<Vec<String>>(), true);
        assert_eq!(sf.hazardous_location_count(), 12);
    }
}

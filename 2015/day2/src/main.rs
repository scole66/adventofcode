use lazy_static::lazy_static;
use regex::Regex;
use std::io;

struct ElfBox {
    width: u32,
    height: u32,
    depth: u32,
}

impl ElfBox {
    fn paper_needed(&self) -> u32 {
        (if self.width >= self.height && self.width >= self.depth {
            self.height * self.depth
        } else if self.height >= self.width && self.height >= self.depth {
            self.width * self.depth
        } else {
            self.width * self.height
        }) + 2 * (self.width * self.height + self.width * self.depth + self.height * self.depth)
    }

    fn ribbon_needed(&self) -> u32 {
        2 * (if self.width >= self.height && self.width >= self.depth {
            self.height + self.depth
        } else if self.height >= self.width && self.height >= self.depth {
            self.width + self.depth
        } else {
            self.width + self.height
        }) + self.width * self.height * self.depth
    }
}

impl TryFrom<&String> for ElfBox {
    type Error = &'static str;
    fn try_from(src: &String) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref ELFBOX_RE: Regex =
                Regex::new(r"^(?P<width>[0-9]+)x(?P<height>[0-9]+)x(?P<depth>[0-9]+)\s*$").unwrap();
        }
        ELFBOX_RE.captures(src).map_or(Err("Malformed box dimensions"), |caps| {
            Ok(ElfBox {
                width: caps.name("width").unwrap().as_str().parse::<u32>().unwrap(),
                height: caps.name("height").unwrap().as_str().parse::<u32>().unwrap(),
                depth: caps.name("depth").unwrap().as_str().parse::<u32>().unwrap(),
            })
        })
    }
}

fn part1(boxes: &[ElfBox]) -> u32 {
    boxes.iter().map(|eb| eb.paper_needed()).sum()
}

fn part2(boxes: &[ElfBox]) -> u32 {
    boxes.iter().map(|eb| eb.ribbon_needed()).sum()
}

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

    let boxes = lines
        .iter()
        .filter_map(|line| ElfBox::try_from(line).ok())
        .collect::<Vec<ElfBox>>();

    println!("Wrapping Paper Needed: {} sq. ft.", part1(&boxes));
    println!("Ribbon Needed: {} ft.", part2(&boxes));

    Ok(())
}

fn main() {
    std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {err:?}");
            1
        }
    });
}

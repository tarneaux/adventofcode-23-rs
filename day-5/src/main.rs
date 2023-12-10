#![feature(iterator_try_collect)]
use color_eyre::eyre;
use color_eyre::eyre::eyre;
use color_eyre::eyre::OptionExt;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let seeds: Vec<u64> = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|v| v.parse())
        .try_collect()
        .unwrap();
    let multimaps: Vec<_> = input
        .split("\n\n")
        .skip(1)
        .map(|v| v.lines().skip(1).collect::<Vec<_>>())
        .map(|v| MultiMap::new_from_lines(v))
        .try_collect()
        .unwrap();
    let seed_locations: Vec<_> = seeds
        .iter()
        .map(|s| multimaps.iter().fold(*s, |previous, m| m.convert(previous)))
        .collect();
    println!("{:?}", seed_locations.iter().min().unwrap());
}

#[derive(Debug)]
struct MultiMap {
    maps: Vec<Map>,
}

impl MultiMap {
    pub fn new_from_lines(lines: Vec<&str>) -> eyre::Result<Self> {
        Ok(Self {
            maps: lines.iter().map(|l| Map::new_from_line(l)).try_collect()?,
        })
    }

    pub fn convert(&self, input: u64) -> u64 {
        self.maps
            .iter()
            .find_map(|m| m.try_convert(input))
            .unwrap_or(input)
    }
}

#[derive(Debug)]
struct Map {
    src_start: u64,
    dest_start: u64,
    len: u64,
}

impl Map {
    pub fn new_from_line(line: &str) -> eyre::Result<Self> {
        let mut line = line.split_whitespace();
        let mut get_one =
            || -> eyre::Result<u64> { Ok(line.next().ok_or_eyre("Not enough words")?.parse()?) };
        let (dest_start, src_start, len) = (get_one()?, get_one()?, get_one()?);
        if !line.next().is_none() {
            return Err(eyre!("Too many words"));
        }
        Ok(Self {
            src_start,
            dest_start,
            len,
        })
    }

    pub fn try_convert(&self, input: u64) -> Option<u64> {
        let input_index = input.checked_sub(self.src_start)?;
        if input_index < self.len {
            Some(self.dest_start + (input_index))
        } else {
            None
        }
    }
}

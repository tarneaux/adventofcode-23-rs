#![feature(iterator_try_collect)]
use color_eyre::eyre;
use color_eyre::eyre::eyre;
use color_eyre::eyre::OptionExt;
use std::collections::HashMap;
use std::ops::RangeInclusive;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let seeds: Vec<u32> = input
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
        .collect();
    println!("{:?}", multimaps);
    let reduced_map = multimaps
        .into_iter()
        .reduce(|a, b| a?.merge_with_next(b?))
        .unwrap()
        .unwrap();
    println!("{:?}", reduced_map);
    let seed_locations: Vec<_> = seeds
        .iter()
        .map(|s| (*s, reduced_map.convert(*s)))
        .collect();
    println!("{:?}", seed_locations)
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

    pub fn convert(&self, input: u32) -> u32 {
        self.maps
            .iter()
            .find_map(|m| m.try_convert(input))
            .unwrap_or(input)
    }

    pub fn merge_with_next(self, next: Self) -> eyre::Result<Self> {
        let my_allowed_inputs: Vec<u32> = (0..100).collect();
        let hm: HashMap<u32, u32> = my_allowed_inputs
            .iter()
            .map(|i| (*i, self.convert(*i)))
            .map(|(i, o)| (i, next.convert(o)))
            .collect();
        Ok(Self {
            maps: vec![Map::HashMap(hm)],
        })
    }
}

#[derive(Debug)]
enum Map {
    Ranges(RangeInclusive<u32>, RangeInclusive<u32>),
    HashMap(HashMap<u32, u32>),
}

impl Map {
    pub fn new_from_line(line: &str) -> eyre::Result<Self> {
        let mut line = line.split_whitespace();
        let mut get_one =
            || -> eyre::Result<u32> { Ok(line.next().ok_or_eyre("Not enough words")?.parse()?) };
        let (dest_range_start, src_range_start, range_len): (u32, u32, u32) =
            (get_one()?, get_one()?, get_one()?);
        if !line.next().is_none() {
            return Err(eyre!("Too many words"));
        }
        Ok(Self::Ranges(
            src_range_start..=(src_range_start + range_len),
            dest_range_start..=(dest_range_start + range_len),
        ))
    }

    pub fn try_convert(&self, input: u32) -> Option<u32> {
        match self {
            Self::Ranges(ir, or) => or.clone().nth(ir.clone().position(|v| v == input)?),
            Self::HashMap(h) => Some(h[&input]),
        }
    }
}

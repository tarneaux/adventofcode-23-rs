#![feature(iterator_try_collect)]
use color_eyre::eyre;
use color_eyre::eyre::OptionExt;
use std::num::ParseIntError;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let cards: Vec<Card> = input
        .lines()
        .map(|line| line.try_into())
        .try_collect()
        .unwrap();
    let scores: Vec<_> = cards.iter().map(|c| c.get_score()).try_collect().unwrap();
    println!("Part 1: {:?}", scores.iter().sum::<u32>());
    let matches: Vec<_> = cards.iter().map(|c| c.get_matching().len()).collect();
    let mut multipliers: Vec<_> = cards.iter().map(|_| 1).collect();
    for i in 0..scores.len() {
        let matching = matches[i];
        let multiplier = multipliers[i];
        for j in 1..=matching {
            let index = i + j;
            if index < multipliers.len() {
                multipliers[index] += multiplier;
            }
        }
    }
    println!("{}", multipliers.iter().sum::<u32>());
}

#[derive(Debug)]

struct Card {
    n: u32,
    winning: Vec<u32>,
    have: Vec<u32>,
}

impl Card {
    pub fn get_matching(&self) -> Vec<u32> {
        self.have
            .iter()
            .filter(|v| self.winning.contains(v))
            .map(|v| v.clone())
            .collect::<Vec<_>>()
    }

    pub fn get_score(&self) -> eyre::Result<u32> {
        let matches: u32 = self.get_matching().iter().count().try_into()?;
        Ok(if matches > 0 {
            2_u32.pow(matches - 1)
        } else {
            0
        })
    }
}

impl TryFrom<&str> for Card {
    type Error = color_eyre::eyre::Error;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let (left, right) = s
            .split_once(":")
            .ok_or_eyre("Could not find ':' in string")?;
        let n: u32 = left
            .split_whitespace()
            .last()
            .ok_or_eyre("Couldn't get the last word on the left of ':' in string")?
            .parse()?;
        let (winning, have) = right
            .split_once("|")
            .ok_or_eyre("Could not find '|' on the left of ':' in string")?;
        let convert_string_to_numbers = |s: &str| -> Result<Vec<u32>, ParseIntError> {
            s.to_owned()
                .split_whitespace()
                .map(|v| v.parse::<u32>())
                .try_collect()
        };
        let (winning, have) = (
            convert_string_to_numbers(winning)?,
            convert_string_to_numbers(have)?,
        );
        Ok(Card { n, winning, have })
    }
}

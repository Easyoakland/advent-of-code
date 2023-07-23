use advent_lib::parse::read_and_leak;
use std::error::Error;

mod data;
mod parse;

mod part1 {
    use super::*;
    use crate::{
        data::{Robot, Round},
        parse::parse_input,
    };
    use std::collections::BTreeMap;

    pub fn run(file_name: &str) -> Result<usize, Box<dyn Error>> {
        let input = read_and_leak(file_name)?;
        let blueprints = parse_input(input)?;
        let starting_rounds = blueprints.into_iter().map(|blueprint| Round {
            blueprint,
            robots: BTreeMap::from([(Robot::Ore, 1)]),
            ..Default::default()
        });
        Ok(starting_rounds
            .into_iter()
            .map(|round| round.max_geodes(10))
            .enumerate()
            .map(|(i, quality_level)| i * usize::from(quality_level))
            .sum())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part 1 answer: {:#?}", part1::run("input.txt")?);
    // println!("Part 2 answer: {:#?}", part2::run("input.txt")?);
    Ok(())
}

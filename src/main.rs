use std::{error::Error, fs};

mod data;
mod parse;
use parse::{parse_final, Span};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part 1 answer: {:#?}", part1::run("input.txt")?);
    Ok(())
}

mod part1 {
    use super::*;
    pub fn run(file: &str) -> Result<u32, Box<dyn Error>> {
        let input_str = fs::read_to_string(file)?;
        let input: Span<'static> = Span::new(Box::leak(Box::new(input_str)));
        let monkeys = parse_final(input)?;
        dbg!(monkeys);

        Ok(0)
    }
}

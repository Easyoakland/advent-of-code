use std::{
    error::Error,
    fs::{self},
};

mod parse;
use parse::{parse_input, Span};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part 1 answer: {:#?}", part1::run("input.txt")?);
    Ok(())
}

mod part1 {
    use nom::error;

    use super::*;
    pub fn run(file: &str) -> Result<u32, Box<dyn Error>> {
        let input = fs::read_to_string(file)?;
        let input = Span::new(&input);
        let (_, monkeys) =
            parse::parse_input::<u8, error::Error<Span>>(input).map_err(|e| e.to_string())?;
        Ok(0)
    }
}

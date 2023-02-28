use advent_15::dbc;
use std::{error::Error, fs};
mod cord;
mod data;
mod parse;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part 1 answer: {:#?}", part1::run("input.txt", 2000000)?);
    // println!("Part 2 answer: {:#?}", part2::run("input.txt")?);
    Ok(())
}

mod part1 {
    use std::collections::HashSet;

    use super::*;
    pub fn run(file: &str, row: isize) -> Result<usize, Box<dyn Error>> {
        let input_str = fs::read_to_string(file)?;
        let input = Box::leak(Box::new(input_str));
        let (_, parsed_input) = parse::parse_input::<isize>(input)?;
        let mut safe_spots = HashSet::new();
        for pair in parsed_input {
            let radius = pair.sensor.manhattan_distance(&pair.beacon);
            dbg!(radius);
            safe_spots.extend(pair.sensor.neumann_neighborhood(radius));
            safe_spots.insert(pair.sensor);
            safe_spots.remove(&pair.beacon);
        }
        Ok(safe_spots.iter().filter(|&x| x.1 == row).count())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1_out_parse() -> Result<(), Box<dyn Error>> {
        let input_str = fs::read_to_string("inputtest.txt")?;
        let input = Box::leak(Box::new(input_str));
        let (_, parsed_input) = parse::parse_input::<isize>(input)?;
        dbc!(parsed_input);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<(), Box<dyn Error>> {
        assert_eq!(part1::run("inputtest.txt", 10)?, 26);
        Ok(())
    }

    // #[test]
    // fn test_part2() -> Result<(), Box<dyn Error>> {
    //     assert_eq!(part2::run("inputtest.txt")?, 93);
    //     Ok(())
    // }
}

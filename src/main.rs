use crate::cord::Cord;
#[allow(unused_imports)]
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
    use itertools::Itertools;

    use super::*;
    use std::collections::HashSet;
    pub fn run(file: &str, row: isize) -> Result<usize, Box<dyn Error>> {
        let input_str = fs::read_to_string(file)?;
        let input = Box::leak(Box::new(input_str));
        let (_, mut parsed_input) = parse::parse_input::<isize>(input)?;
        let mut safe_spots = HashSet::new();
        for pair in &mut parsed_input {
            let radius = pair.sensor.manhattan_distance(&pair.beacon);
            // If the scan range reaches onto the row of interest.
            // Project the leftover range onto the row.
            let leftover =
                radius.saturating_sub(dbc!(isize::try_from(row.abs_diff(pair.sensor.1)).unwrap()));
            dbc!(&pair, &radius, &leftover);
            if leftover >= 0 {
                safe_spots.extend(
                    ((Cord(pair.sensor.0, row) - Cord(leftover, 0))
                        .interpolate(&(Cord(pair.sensor.0, row) + Cord(leftover, 0))))
                    .into_iter()
                    .map(|x| x.0),
                )
            }
            dbc!(&safe_spots.iter().sorted().collect::<Vec<_>>());
            // Scanners are definitely safe.
            if pair.sensor.1 == row {
                safe_spots.insert(pair.sensor.0);
            }
        }
        for pair in parsed_input {
            if pair.beacon.1 == row {
                // Beacons are definitely beacons. Remove them if they are in the row.
                safe_spots.remove(&pair.beacon.0);
            }
        }
        dbc!(&safe_spots.iter().sorted().collect::<Vec<_>>());

        // Only care about safe spots on the same row
        Ok(safe_spots.len())
    }

    #[allow(dead_code)]
    pub fn run_naive(file: &str, row: isize) -> Result<usize, Box<dyn Error>> {
        let input_str = fs::read_to_string(file)?;
        let input = Box::leak(Box::new(input_str));
        let (_, parsed_input) = parse::parse_input::<isize>(input)?;
        let mut safe_spots = HashSet::new();
        for pair in parsed_input {
            let radius = pair.sensor.manhattan_distance(&pair.beacon);
            let neighbors = pair.sensor.neumann_neighborhood(radius);
            safe_spots.extend(neighbors.filter(|&x| x.1 == row));
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

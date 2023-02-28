use std::{error::Error, fs};
mod cord;
mod data;
mod parse;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part 1 answer: {:#?}", part1::run("input.txt")?);
    println!("Part 2 answer: {:#?}", part2::run("input.txt")?);
    Ok(())
}

mod part1 {
    use crate::{cord::Cord, data::Sand};
    use std::collections::HashSet;
    const SAND_START: Cord<usize> = Cord(500, 0);

    use super::*;
    pub fn run(file: &str) -> Result<usize, Box<dyn Error>> {
        let input_str = fs::read_to_string(file)?;
        let input = Box::leak(Box::new(input_str));
        let (_, parsed_input) = parse::parse_input(input)?;

        // Add all positions of any rock to the rocks set.
        let mut rocks: HashSet<Cord<usize>> = HashSet::new();
        for connected in parsed_input {
            for cord_pair in connected.windows(2) {
                rocks.extend(&cord_pair[0].interpolate(&cord_pair[1]));
            }
        }

        // Find bottom level
        let bottom = rocks.iter().max_by_key(|&x| x.1).unwrap().1;

        let mut sands = HashSet::new();
        'newsand: loop {
            let mut sand = Sand { pos: SAND_START };
            while sand.fall(&rocks, &sands) {
                // If sand falls off the edge stop adding sand.
                if sand.pos.1 > bottom {
                    break 'newsand;
                }
            }
            sands.insert(sand.pos);
        }
        Ok(sands.len())
    }
}

mod part2 {
    use crate::{cord::Cord, data::Sand};
    use std::collections::HashSet;
    const SAND_START: Cord<usize> = Cord(500, 0);

    use super::*;
    pub fn run(file: &str) -> Result<usize, Box<dyn Error>> {
        let input_str = fs::read_to_string(file)?;
        let input = Box::leak(Box::new(input_str));
        let (_, parsed_input) = parse::parse_input(input)?;

        // Add all positions of any rock to the rocks set.
        let mut rocks: HashSet<Cord<usize>> = HashSet::new();
        for connected in parsed_input {
            for cord_pair in connected.windows(2) {
                rocks.extend(&cord_pair[0].interpolate(&cord_pair[1]));
            }
        }

        // Find bottom level. It is 2 below bottom rock.
        let bottom = rocks.iter().max_by_key(|&x| x.1).unwrap().1 + 2;
        let left = rocks.iter().min_by_key(|&x| x.0).unwrap().0;
        let right = rocks.iter().max_by_key(|&x| x.0).unwrap().0;

        let bottom_left = Cord(left, bottom);
        let bottom_right = Cord(right, bottom);

        // Add the floor
        rocks.extend(bottom_left.interpolate(&bottom_right));

        let mut sands = HashSet::new();
        'newsand: loop {
            let mut sand = Sand { pos: SAND_START };
            while sand.fall(&rocks, &sands) {
                // If sand falls off covers hold stop adding sand.
                if sand.pos == SAND_START {
                    break 'newsand;
                }
            }
            sands.insert(sand.pos);
        }
        Ok(sands.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1_out_parse() -> Result<(), Box<dyn Error>> {
        let input_str = fs::read_to_string("inputtest.txt")?;
        let input = Box::leak(Box::new(input_str));
        let (_, parsed_input) = parse::parse_input::<usize>(input)?;
        dbg!(parsed_input);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<(), Box<dyn Error>> {
        assert_eq!(part1::run("inputtest.txt")?, 24);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), Box<dyn Error>> {
        assert_eq!(part2::run("inputtest.txt")?, 93);
        Ok(())
    }
}

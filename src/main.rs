use crate::data::SandPosType;
use crate::{cord::Cord, data::Sand};
use std::collections::HashSet;
use std::fs::OpenOptions;
use std::{error::Error, fs, io::Write};
mod cord;
mod data;
mod parse;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part 1 answer: {:#?}", part1::run("input.txt")?);
    println!("Part 2 answer: {:#?}", part2::run("input.txt")?);
    Ok(())
}

mod part1 {
    const SAND_START: Cord<SandPosType> = Cord(500, 0);
    use super::*;
    pub fn run(file: &str) -> Result<usize, Box<dyn Error>> {
        let input_str = fs::read_to_string(file)?;
        let input = Box::leak(Box::new(input_str));
        let (_, parsed_input) = parse::parse_input(input)?;

        // Add all positions of any rock to the rocks set.
        let mut rocks: HashSet<Cord<SandPosType>> = HashSet::new();
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
    const SAND_START: Cord<SandPosType> = Cord(500, 0);

    use super::*;
    pub fn run(file: &str) -> Result<usize, Box<dyn Error>> {
        let input_str = fs::read_to_string(file)?;
        let input = Box::leak(Box::new(input_str));
        let (_, parsed_input) = parse::parse_input(input)?;

        // Add all positions of any rock to the rocks set.
        let mut rocks: HashSet<Cord<SandPosType>> = HashSet::new();
        for connected in parsed_input {
            for cord_pair in connected.windows(2) {
                rocks.extend(&cord_pair[0].interpolate(&cord_pair[1]));
            }
        }

        // Find bottom level. It is 2 below bottom rock.
        let bottom = rocks.iter().max_by_key(|&x| x.1).unwrap().1 + 2;
        // 200's are buffer so overrun doesn't go to the void.
        let left = rocks.iter().min_by_key(|&x| x.0).unwrap().0 - 200;
        let right = rocks.iter().max_by_key(|&x| x.0).unwrap().0 + 200;

        let bottom_left = Cord(left, bottom);
        let bottom_right = Cord(right, bottom);

        // Add the floor
        rocks.extend(bottom_left.interpolate(&bottom_right));

        let mut sands = HashSet::new();
        'newsand: loop {
            let mut sand = Sand { pos: SAND_START };

            while sand.fall(&rocks, &sands) {
                // If sand falls off covers hold stop adding sand.
                if sand.pos.1 == SAND_START.1 {
                    break 'newsand;
                }
            }
            sands.insert(sand.pos);
            // Check for sand that doesn't get to fall.
            if sand.pos == SAND_START {
                break 'newsand;
            }
        }
        save_state("out.txt", left, right, bottom, &sands, &rocks);
        Ok(sands.len())
    }
}

fn save_state(
    file: &str,
    left: SandPosType,
    right: SandPosType,
    bottom: SandPosType,
    sands: &HashSet<Cord<SandPosType>>,
    rocks: &HashSet<Cord<SandPosType>>,
) {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file)
        .unwrap();
    for y in 0..=bottom {
        for x in left..=right {
            if rocks.contains(&Cord(x, y)) {
                write!(file, "#").unwrap();
            } else if sands.contains(&Cord(x, y)) {
                write!(file, "o").unwrap();
            } else {
                write!(file, " ").unwrap();
            }
        }
        writeln!(file, "").unwrap();
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

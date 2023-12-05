use crate::data::{Packet, Pair};
use itertools::Itertools;
use std::error::Error;
use std::fs;
mod data;
mod parse;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part 1 answer: {:#?}", part1::run("input.txt")?);
    println!("Part 2 answer: {:#?}", part2::run("input.txt")?);
    Ok(())
}

mod part1 {
    use super::*;
    pub fn run(file: &str) -> Result<usize, Box<dyn Error>> {
        let input_str = fs::read_to_string(file)?;
        let input = Box::leak(Box::new(input_str));
        let (_input, pairs) = parse::parse_input(input)?;
        Ok(pairs
            .iter()
            .enumerate()
            // 1 indexed
            .map(|(mut i, x)| {
                i += 1;
                (i, x)
            })
            // for each "correctly ordered" (meaning <)
            .filter(|(_, pair)| std::cmp::Ordering::Less == pair.left.cmp(&pair.right))
            // sum its index
            .fold(0, |acc, x| acc + x.0))
    }
}

mod part2 {
    use super::*;
    pub fn run(file: &str) -> Result<usize, Box<dyn Error>> {
        let input_str = fs::read_to_string(file)?;
        let input = Box::leak(Box::new(input_str));
        let divider_packet_1 = Packet::List(vec![Packet::List(vec![Packet::Integer(2)])]);
        let divider_packet_2 = Packet::List(vec![Packet::List(vec![Packet::Integer(6)])]);
        let (_input, mut pairs) = parse::parse_input(input)?;
        pairs.push(Pair {
            left: divider_packet_1.clone(),
            right: divider_packet_2.clone(),
        });
        // Pull out elements of the pairs.
        let sorted_packets = pairs
            .into_iter()
            .flat_map(|pair| vec![pair.left, pair.right])
            .sorted()
            .collect::<Vec<_>>();
        let loc1 = sorted_packets
            .iter()
            .position(|x| x == &divider_packet_1)
            .unwrap()
            + 1;
        let loc2 = sorted_packets
            .iter()
            .position(|x| x == &divider_packet_2)
            .unwrap()
            + 1;
        Ok(loc1 * loc2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1_out_parse() -> Result<(), Box<dyn Error>> {
        let input_str = fs::read_to_string("inputtest.txt")?;
        let input = Box::leak(Box::new(input_str));
        let (_, pairs) = parse::parse_input(input)?;
        // pairs.iter().for_each(|x| print!("{x}\n"));
        for pair in pairs {
            print!("{}{:?}\n", pair, pair.left.cmp(&pair.right));
        }
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<(), Box<dyn Error>> {
        assert_eq!(part1::run("inputtest.txt")?, 13);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), Box<dyn Error>> {
        assert_eq!(part2::run("inputtest.txt")?, 140);
        Ok(())
    }
}

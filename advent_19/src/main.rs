use advent_lib::parse::read_and_leak;
use std::error::Error;

mod data;
mod parse;

mod part1 {
    use super::*;
    use crate::{
        data::{max_geodes, Robot, Round},
        parse::parse_input,
    };
    use std::{cell::Cell, collections::BTreeMap, rc::Rc};

    pub fn run(file_name: &str) -> Result<usize, Box<dyn Error>> {
        let input = read_and_leak(file_name)?;
        let blueprints = parse_input(input)?;
        let starting_rounds = blueprints.into_iter().map(|blueprint| {
            [
                Round {
                    blueprint: blueprint.clone(),
                    robots: BTreeMap::from([(Robot::Ore, 1)]),
                    target: Robot::Ore,
                    ..Default::default()
                },
                Round {
                    blueprint,
                    robots: BTreeMap::from([(Robot::Ore, 1)]),
                    target: Robot::Clay,
                    ..Default::default()
                },
            ]
        });
        Ok({
            let ret = starting_rounds
                .into_iter()
                .map(|round| {
                    (
                        round[0].blueprint.id,
                        max_geodes(round[0].clone(), 24, Rc::new(Cell::new(0))),
                    )
                        .max((
                            round[1].blueprint.id,
                            max_geodes(round[1].clone(), 24, Rc::new(Cell::new(0))),
                        ))
                })
                .map(|(id, quality_level)| id * usize::from(quality_level.unwrap()))
                .sum();

            unsafe { dbg!(data::CNT) };
            ret
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part 1 answer: {:#?}", part1::run("input.txt")?);
    // println!("Part 2 answer: {:#?}", part2::run("input.txt")?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_lib::dbc;

    #[test]
    fn test_part1() -> Result<(), Box<dyn Error>> {
        assert_eq!(part1::run("inputtest.txt")?, 33);
        Ok(())
    }

    #[test]
    fn part1_ans() -> Result<(), Box<dyn Error>> {
        assert!(dbc!(part1::run("input.txt")?) < 12306);
        assert!(part1::run("input.txt")? < 3548);
        assert_eq!(part1::run("input.txt")?, 3542);
        Ok(())
    }

    // #[test]
    // fn test_part2() -> Result<(), Box<dyn Error>> {
    //     assert_eq!(part2::run("inputtest.txt")?, 58);
    //     Ok(())
    // }

    // #[test]
    // fn part2_ans() -> Result<(), Box<dyn Error>> {
    //     assert_eq!(part2::run("input.txt")?, 2080);
    //     Ok(())
    // }
}

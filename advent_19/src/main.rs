use advent_lib::parse::read_and_leak;
use std::error::Error;

mod data {
    use derive_more::{Add, AddAssign, Mul, Sub, SubAssign};
    use enum_iterator::Sequence;
    use std::{
        cmp::Ordering,
        collections::{BTreeMap, BTreeSet},
        time::Duration,
    };

    #[derive(
        Clone,
        Copy,
        Debug,
        Default,
        Add,
        AddAssign,
        Sub,
        SubAssign,
        Mul,
        Sequence,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct Resource {
        pub ore: u8,
        pub clay: u8,
        pub obsidian: u8,
        pub geode: u8,
    }

    impl PartialOrd for Resource {
        // Equal only if all resources are the same. If not equal:
        // Resource is less than other other only if consuming that many resources from other is possible and vice versa.
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            match (
                self.ore.cmp(&other.ore),
                self.clay.cmp(&other.clay),
                self.obsidian.cmp(&other.obsidian),
                self.geode.cmp(&other.geode),
            ) {
                (Ordering::Equal, Ordering::Equal, Ordering::Equal, Ordering::Equal) => {
                    Some(Ordering::Equal)
                }
                (
                    Ordering::Less | Ordering::Equal,
                    Ordering::Less | Ordering::Equal,
                    Ordering::Less | Ordering::Equal,
                    Ordering::Less | Ordering::Equal,
                ) => Some(Ordering::Less),
                (
                    Ordering::Greater | Ordering::Equal,
                    Ordering::Greater | Ordering::Equal,
                    Ordering::Greater | Ordering::Equal,
                    Ordering::Greater | Ordering::Equal,
                ) => Some(Ordering::Greater),
                _ => None,
            }
        }
    }

    #[derive(Clone, Debug, Default, Hash, PartialEq, Eq)]
    pub struct Blueprint {
        pub id: usize,
        pub ore_robot_cost: Resource,
        pub clay_robot_cost: Resource,
        pub obsidian_robot_cost: Resource,
        pub geode_robot_cost: Resource,
    }

    impl Blueprint {
        pub fn construct_cost(&self, robot: &Robot) -> Resource {
            match robot {
                Robot::Ore => self.ore_robot_cost,
                Robot::Clay => self.clay_robot_cost,
                Robot::Obsidian => self.obsidian_robot_cost,
                Robot::Geode => self.geode_robot_cost,
            }
        }

        pub fn affordable_robots(&self, available_resources: Resource) -> Vec<Robot> {
            let res: Vec<Robot> = enum_iterator::all()
                .filter(|x| self.construct_cost(x) <= available_resources)
                .collect();
            res
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Sequence)]
    pub enum Robot {
        Ore,
        Clay,
        Obsidian,
        Geode,
    }

    impl Robot {
        pub fn gen(&self) -> Resource {
            match self {
                Robot::Ore => Resource {
                    ore: 1,
                    ..Default::default()
                },
                Robot::Clay => Resource {
                    clay: 1,
                    ..Default::default()
                },
                Robot::Obsidian => Resource {
                    obsidian: 1,
                    ..Default::default()
                },
                Robot::Geode => Resource {
                    geode: 1,
                    ..Default::default()
                },
            }
        }
    }

    #[derive(Clone, Debug, Default, Hash, PartialEq, Eq)]
    pub struct Round {
        pub minute: u8,
        pub blueprint: Blueprint,
        pub resources: Resource,
        pub robots: BTreeMap<Robot, u8>,
    }

    impl Round {
        pub fn do_round_in_place(&mut self, robots_to_create: Vec<Robot>) {
            // Step 0: increment minute
            self.minute += 1;
            // Step 1: spend to create robots
            for robot in &robots_to_create {
                self.resources -= self.blueprint.construct_cost(&robot);
            }
            // Step 2: robots get resources
            for (robot, cnt) in &self.robots {
                self.resources += robot.gen() * cnt
            }
            // Step 3: create robots that resources were spent to create
            for robot in robots_to_create {
                let entry = self.robots.entry(robot).or_default();
                *entry += 1;
            }
        }

        pub fn do_round(&self, robots_to_create: Vec<Robot>) -> Self {
            let mut new_rnd = self.clone();
            new_rnd.do_round_in_place(robots_to_create);
            new_rnd
        }

        pub fn possible_moves(&self) -> BTreeSet<Vec<Robot>> {
            let mut new_moves = BTreeSet::from([Vec::new()]);
            let mut change = true;
            // As long as the state changed since last iteration keep iterating.
            while change {
                change = false;
                let mut old = new_moves.clone();
                // Take new moves and add further moves as needed.
                while let Some(mov) = old.pop_first() {
                    println!("Move: {:?}", mov);
                    std::thread::sleep(Duration::from_millis(1));

                    // Resources left is the number of resources available minus the cost the move already has.
                    let resources_left = self.resources
                        - mov
                            .iter()
                            .map(|x| self.blueprint.construct_cost(x))
                            .reduce(|acc, x| acc + x)
                            .unwrap_or_default();

                    // Add robots that can be constructed after the other robots in the move as new moves.
                    for robot in self.blueprint.affordable_robots(resources_left) {
                        let mut new_move = mov.clone();
                        new_move.push(robot);
                        new_move.sort(); // deduplicate different order same move ex ab and ba
                        change |= new_moves.insert(new_move);
                    }
                }
            }
            new_moves
        }

        /// Maximum possible geodes with a given blueprint given the current round state.
        pub fn max_geodes(&self, last_minute: u8) -> u8 {
            let indent = (0..self.minute).map(|_| " ").collect::<String>();
            println!(
                "{}Min: {}, Robots: {:?}, {:?}, {:?}",
                indent, self.minute, self.robots, self.resources, self.blueprint
            );
            if self.minute >= last_minute {
                return self.resources.geode;
            }

            self.possible_moves()
                .iter()
                .cloned()
                .map(|robots_to_create| self.do_round(robots_to_create).max_geodes(last_minute))
                .max()
                .expect("Nonempty")
        }
    }
}

mod parse {
    use crate::data::{Blueprint, Resource};
    use std::{borrow::Borrow, char, str::FromStr};
    use yap::{one_of, types::StrTokens, IntoTokens, Tokens};

    pub fn tag<Input, T>(input: &mut Input, tag: T) -> Option<T>
    where
        Input: Tokens,
        <Input as Tokens>::Item: PartialEq,
        T: IntoIterator + Clone,
        <T as IntoIterator>::Item: Borrow<<Input as Tokens>::Item>,
    {
        if input.tokens(tag.clone()) {
            Some(tag)
        } else {
            None
        }
    }

    // Parses at least 1 digit.
    pub fn digit1<I, O>(input: &mut I) -> Option<Result<O, <O as FromStr>::Err>>
    where
        I: Tokens<Item = char>,
        O: FromStr,
    {
        let to_parse = input.tokens_while(|t| t.is_numeric()).collect::<String>();
        if to_parse.is_empty() {
            None
        } else {
            Some(to_parse.parse::<O>())
        }
    }

    /// Parses a line ending of either "\n" (like on linux)  or "\r\n" (like on windows)
    fn line_ending(tokens: &mut impl Tokens<Item = char>) -> Option<&str> {
        yap::one_of!(tokens;
            tokens.optional(|t| t.token('\n').then_some("\n")),
            tokens.optional(|t| t.tokens("\r\n".chars()).then_some("\r\n")),
        )
    }

    #[derive(Debug, thiserror::Error)]
    pub enum ParseError<Item> {
        #[error("Input wasn't fully consumed. Remainder: {0:?}")]
        AllConsuming(Vec<Item>),
    }

    /// Attempts to parse all remainder of input until next None. Consumes nothing on fail.
    pub fn all_consuming<I, O, F>(input: &mut I, parser: F) -> Result<O, ParseError<I::Item>>
    where
        I: Tokens,
        F: FnOnce(&mut I) -> O,
    {
        let before_consuming = input.location();
        let res = parser(input);
        // Check nothing comes after
        match input.next() {
            None => Ok(res),
            Some(x) => {
                let res = Err(ParseError::AllConsuming(
                    std::iter::once(x).chain(input.as_iter()).collect(),
                ));
                input.set_location(before_consuming);
                res
            }
        }
    }

    pub fn sub_resource(input: &mut StrTokens, subresource: &str) -> Option<u8> {
        {
            let res = Some(digit1(input)?.ok()?);
            let _ = tag(input, " ".chars())?;
            let _ = tag(input, subresource.chars())?;
            if one_of!(input;
                input.tokens(" and ".chars()).then(|| ()),
                input.tokens(".".chars()).then(|| ()),
            )
            .is_some()
            {
                return res;
            };

            None
        }
    }

    pub fn resource(input: &mut StrTokens) -> Option<Resource> {
        Some(Resource {
            ore: input
                .optional(|t| sub_resource(t, "ore"))
                .unwrap_or_default(),
            clay: input
                .optional(|t| sub_resource(t, "clay"))
                .unwrap_or_default(),
            obsidian: input
                .optional(|t| sub_resource(t, "obsidian"))
                .unwrap_or_default(),
            geode: 0,
        })
    }

    pub fn robot_cost(input: &mut StrTokens, robot_name: &str) -> Option<Resource> {
        let _ = tag(input, " Each ".chars())?;
        let _ = tag(input, robot_name.chars())?;
        let _ = tag(input, " robot costs ".chars())?;
        let cost = resource(input)?;
        Some(cost)
    }

    pub fn blueprint(input: &mut StrTokens) -> Option<Blueprint> {
        let (_, id, _) = (
            tag(input, "Blueprint ".chars())?,
            digit1(input)?.ok()?,
            tag(input, ":".chars())?,
        );
        let ore_robot_cost = robot_cost(input, "ore")?;
        let clay_robot_cost = robot_cost(input, "clay")?;
        let obsidian_robot_cost = robot_cost(input, "obsidian")?;
        let geode_robot_cost = robot_cost(input, "geode")?;
        Some(Blueprint {
            id,
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_cost,
            geode_robot_cost,
        })
    }

    pub fn parse_input(input: &str) -> Result<Vec<Blueprint>, ParseError<char>> {
        all_consuming(&mut input.into_tokens(), |t| {
            let res = t
                .sep_by(blueprint, |t| line_ending(t).is_some())
                .collect::<Vec<_>>();
            line_ending(t);
            res
        })
    }
}

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
            .map(|round| round.max_geodes(3))
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

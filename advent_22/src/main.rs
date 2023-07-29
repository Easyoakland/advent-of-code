use std::error::Error;

mod data {
    use advent_lib::{cord::NDCord, parse::yap::digit1};
    use std::{collections::BTreeMap, str::FromStr};
    use yap::{types::StrTokens, IntoTokens, Tokens};
    pub type Val = usize;
    pub type Pos = NDCord<Val, 2>;

    #[derive(Clone, Copy, Debug)]
    pub enum PosKind {
        Open,
        Wall,
    }

    impl TryFrom<char> for PosKind {
        type Error = char;

        fn try_from(value: char) -> Result<Self, Self::Error> {
            use PosKind::*;
            match value {
                '.' => Ok(Open),
                '#' => Ok(Wall),
                c => Err(c),
            }
        }
    }

    impl FromStr for PosKind {
        type Err = char;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            s.chars().next().expect("TODO").try_into()
        }
    }

    pub type Map = BTreeMap<Pos, PosKind>;

    #[derive(Clone, Copy, Debug)]
    pub enum Rotation {
        Left,
        Right,
    }

    impl TryFrom<char> for Rotation {
        type Error = char;

        fn try_from(value: char) -> Result<Self, Self::Error> {
            use Rotation::*;
            match value {
                'R' => Ok(Right),
                'L' => Ok(Left),
                c => Err(c),
            }
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub enum Move {
        Forward(usize),
        Rotate(Rotation),
    }

    impl<'a> TryFrom<&mut StrTokens<'a>> for Move {
        type Error = ();

        fn try_from(value: &mut StrTokens<'a>) -> Result<Self, Self::Error> {
            digit1(value)
                .map(|x| Move::Forward(x.expect("parse error")))
                .ok_or(value)
                .or_else(|x| {
                    x.next()
                        .ok_or(())
                        .and_then(|x| Rotation::try_from(x).map_err(|_| ()))
                        .and_then(|x| Ok(Move::Rotate(x)))
                })
        }
    }

    impl FromStr for Move {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let tok = &mut s.into_tokens();
            tok.try_into()
        }
    }
}

mod parse {
    use crate::data::{Map, Move, PosKind};
    use advent_lib::parse::yap::{all_consuming, ParseError};
    use std::collections::BTreeMap;
    use yap::{types::StrTokens, IntoTokens, Tokens};

    pub fn map(input: &mut StrTokens) -> Map {
        let mut out = BTreeMap::new();
        let origin = [1, 1];
        let mut current_pos = origin.clone();
        for c in input.tokens_while(|x| matches!(x, '\n' | ' ' | '.' | '#')) {
            if c == '\n' {
                current_pos[0] = origin[0];
                current_pos[1] += 1;
                continue;
            }
            if c == ' ' {
                current_pos[0] += 1;
                continue;
            }
            match PosKind::try_from(c) {
                Ok(x) => {
                    out.insert(current_pos.into(), x);
                    current_pos[0] += 1;
                }
                Err(_) => break,
            }
        }
        out
    }

    pub fn moves(input: &mut StrTokens) -> Vec<Move> {
        let mut out = Vec::new();
        loop {
            match input.try_into() {
                Ok(x) => out.push(x),
                Err(_) => break,
            }
        }
        out
    }

    pub fn parse_input(input: &str) -> Result<(Map, Vec<Move>), ParseError<char>> {
        all_consuming(&mut input.into_tokens(), |t| (map(t), moves(t)))
    }
}

mod part1 {
    use super::*;
    use crate::{
        data::{Pos, Val},
        parse::parse_input,
    };
    use advent_lib::parse::read_and_leak;

    pub fn run(file_name: &str) -> Result<Val, Box<dyn Error>> {
        let input = read_and_leak(file_name)?;
        let (map, moves) = parse_input(input)?;
        let extents = Pos::extents_iter(map.iter().map(|x| *x.0));
        advent_lib::dbc!(extents);
        todo!()
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

    #[test]
    fn test_part1() -> Result<(), Box<dyn Error>> {
        assert_eq!(part1::run("inputtest.txt")?, 6032);
        Ok(())
    }

    // #[test]
    // fn part1_ans() -> Result<(), Box<dyn Error>> {
    //     assert_eq!((part1::run("input.txt")? as u64), 31017034894002);
    //     Ok(())
    // }

    // #[test]
    // fn test_part2() -> Result<(), Box<dyn Error>> {
    //     assert_eq!((part2::run("inputtest.txt")? as u64), 301);
    //     Ok(())
    // }

    // #[test]
    // fn part2_ans() -> Result<(), Box<dyn Error>> {
    //     assert!((part2::run("input.txt")? as u64) < 3555057453232);
    //     assert_eq!((part2::run("input.txt")? as u64), 3555057453229);
    //     Ok(())
    // }
}

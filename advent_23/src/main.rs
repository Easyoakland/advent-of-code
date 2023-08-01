use std::error::Error;

mod data {
    use advent_lib::cord::NDCord;
    use enum_iterator::{All, Sequence};
    use std::iter::Cycle;

    pub type Val = isize;
    pub type Pos = NDCord<Val, 2>;

    #[derive(Clone, Copy, Debug, Sequence)]
    pub enum Dir {
        North,
        South,
        East,
        West,
    }

    #[derive(Clone, Debug)]
    pub struct Elf {
        next_dir: Cycle<All<Dir>>,
    }

    impl Elf {
        pub fn new() -> Self {
            Elf {
                next_dir: enum_iterator::all().cycle(),
            }
        }
    }

    impl Iterator for Elf {
        type Item = Dir;

        fn next(&mut self) -> Option<Self::Item> {
            self.next_dir.next()
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (usize::MAX, None)
        }
    }

    impl Default for Elf {
        fn default() -> Self {
            Self::new()
        }
    }
}

mod parse {
    use crate::data::{Elf, Pos};
    use advent_lib::parse::yap::{all_consuming, line_ending, ParseError};
    use std::collections::BTreeMap;
    use yap::{IntoTokens, Tokens};

    pub fn elf(input: &mut impl Tokens<Item = char>) -> Option<Elf> {
        input.token('#').then(|| Elf::new())
    }

    pub fn initial_map(input: &mut impl Tokens<Item = char>) -> BTreeMap<Pos, Elf> {
        let mut out = BTreeMap::new();
        let mut cursor = Pos::default();
        loop {
            loop {
                if let Some(elf) = elf(input) {
                    out.insert(cursor, elf);
                    cursor[0] += 1;
                } else if input.token('.') {
                    cursor[0] += 1;
                } else {
                    break;
                }
            }
            if let None = line_ending(input) {
                break;
            }
            cursor[0] = 0;
            cursor[1] += 1;
        }
        out
    }

    pub fn parse_input(input: &str) -> Result<BTreeMap<Pos, Elf>, ParseError<char>> {
        all_consuming(&mut input.into_tokens(), initial_map)
    }
}

mod part1 {
    use super::*;
    use crate::{data::Val, parse::parse_input};
    use advent_lib::parse::read_and_leak;

    pub fn run(file_name: &str) -> Result<Val, Box<dyn Error>> {
        let input = read_and_leak(file_name)?;
        let map = parse_input(input)?;
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
        assert_eq!(part1::run("inputtest.txt")?, 110);
        Ok(())
    }

    // #[test]
    // fn part1_ans() -> Result<(), Box<dyn Error>> {
    //     assert!(part1::run("input.txt")? > 61338);
    //     assert_eq!(part1::run("input.txt")?, 126350);
    //     Ok(())
    // }

    // #[test]
    // fn test_part2() -> Result<(), Box<dyn Error>> {
    //     assert_eq!(part2::run("inputtest.txt")?, 5031);
    //     Ok(())
    // }

    // #[test]
    // fn part2_ans() -> Result<(), Box<dyn Error>> {
    //     assert_eq!(part2::run("input.txt")?, 129339);
    //     Ok(())
    // }
}

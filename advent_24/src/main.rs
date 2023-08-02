use std::error::Error;
mod data {
    use advent_lib::dir::Dir;
    use ndarray::Array2;
    pub type Map = Array2<Cell>;

    #[derive(Clone, Debug)]
    pub enum Cell {
        Wall,
        Floor,
        Blizzards(Vec<Dir>),
    }
}

mod parse {
    use crate::data::{Cell, Map};
    use advent_lib::{
        dir::Dir,
        parse::yap::{all_consuming, line_ending, AllConsuming},
    };
    use ndarray::ArrayBase;
    use yap::{IntoTokens, Tokens};

    pub fn initial_map(input: &mut impl Tokens<Item = char>) -> Map {
        let mut data = Vec::new();
        // Is first line
        let mut first = true;
        // 1 cell will be open instead of wall (entrance). Start count from 1 to handle.
        let mut rows = 1;
        let mut cols = 0;
        loop {
            loop {
                if input.token('#') {
                    data.push(Cell::Wall);
                    if first {
                        rows += 1;
                    }
                } else if input.token('.') {
                    data.push(Cell::Floor)
                } else if input.token('^') {
                    data.push(Cell::Blizzards(vec![Dir::Up]))
                } else if input.token('<') {
                    data.push(Cell::Blizzards(vec![Dir::Left]))
                } else if input.token('>') {
                    data.push(Cell::Blizzards(vec![Dir::Right]))
                } else if input.token('v') {
                    data.push(Cell::Blizzards(vec![Dir::Down]))
                } else {
                    first = false;
                    break;
                }
            }
            if line_ending(input).is_some() {
                cols += 1;
            } else {
                break;
            }
        }
        advent_lib::dbc!(data.len(), rows, cols, &data);
        ArrayBase::from_shape_vec((cols, rows), data)
            .expect("Valid dimensions")
            .reversed_axes() // make x axis first number and y axis second number
    }

    pub fn parse_input(input: &str) -> Result<Map, AllConsuming<String>> {
        all_consuming(&mut input.into_tokens(), initial_map)
    }
}

mod part1 {
    use super::*;
    use crate::parse::parse_input;
    use advent_lib::parse::read_and_leak;

    pub fn run(file_name: &str) -> Result<usize, Box<dyn Error>> {
        let input = read_and_leak(file_name)?;
        let map = parse_input(input)?;
        advent_lib::dbc!(&map);
        dbg!(&map[[1, 1]], &map[[5, 1]], &map[[1, 5]]);
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
        assert_eq!(part1::run("inputtest.txt")?, 18);
        Ok(())
    }

    //     #[test]
    //     fn part1_ans() -> Result<(), Box<dyn Error>> {
    //         assert!(part1::run("input.txt")? > 61338);
    //         assert_eq!(part1::run("input.txt")?, 126350);
    //         Ok(())
    //     }

    //     #[test]
    //     fn test_part2() -> Result<(), Box<dyn Error>> {
    //         assert_eq!(part2::run("inputtest.txt")?, 5031);
    //         Ok(())
    //     }

    //     #[test]
    //     fn part2_ans() -> Result<(), Box<dyn Error>> {
    //         assert_eq!(part2::run("input.txt")?, 129339);
    //         Ok(())
    //     }
}

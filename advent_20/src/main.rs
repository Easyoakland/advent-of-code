use std::error::Error;

mod parse {
    use advent_lib::parse::yap::{all_consuming, line_ending, ParseError};
    use std::str::FromStr;
    use yap::{IntoTokens, Tokens};
    type Cord = isize;

    pub fn signed_digit1<I, O>(input: &mut I) -> Option<Result<O, <O as FromStr>::Err>>
    where
        I: Tokens<Item = char>,
        O: FromStr + std::fmt::Debug,
        <O as FromStr>::Err: std::fmt::Debug,
    {
        let to_parse = input
            .tokens_while(|&t| t.is_numeric() || t == '+' || t == '-')
            .collect::<String>();
        if to_parse.is_empty() {
            None
        } else {
            Some(to_parse.parse::<O>())
        }
    }

    pub fn parse_input(input: &str) -> Result<Vec<Cord>, ParseError<char>> {
        all_consuming(&mut input.into_tokens(), |t| {
            let res = t
                .sep_by(signed_digit1, |t| line_ending(t).is_some())
                .map(|x| x.unwrap())
                .collect::<Vec<_>>();
            line_ending(t);
            res
        })
    }
}

mod part1 {
    use super::*;
    use crate::parse::parse_input;
    use advent_lib::parse::read_and_leak;

    pub fn run(file_name: &str) -> Result<isize, Box<dyn Error>> {
        let input = read_and_leak(file_name)?;
        let cords = parse_input(input)?;

        // Each has (id, value)
        let mut identity_cords = cords.into_iter().enumerate().collect::<Vec<_>>();
        let original = identity_cords.clone();
        for (i, cord) in original {
            let pos = identity_cords
                .iter()
                .position(|x| x.0 == i)
                .expect("Same length");
            let element = identity_cords.remove(pos);
            identity_cords.insert(
                (pos + usize::try_from(cord.rem_euclid(
                    isize::try_from(identity_cords.len()).expect("Too long for isize"),
                ))
                .expect("not negative"))
                .rem_euclid(identity_cords.len()),
                element,
            );
        }
        let pos_0 = identity_cords
            .iter()
            .position(|x| x.1 == 0)
            .expect("0 pivot exists");
        Ok(identity_cords[(pos_0 + 1000) % identity_cords.len()].1
            + identity_cords[(pos_0 + 2000) % identity_cords.len()].1
            + identity_cords[(pos_0 + 3000) % identity_cords.len()].1)
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
        assert_eq!(part1::run("inputtest.txt")?, 3);
        Ok(())
    }

    // #[test]
    // fn part1_ans() -> Result<(), Box<dyn Error>> {
    //     assert_eq!(part1::run("input.txt")?, 1466);
    //     Ok(())
    // }

    // #[test]
    // fn test_part2() -> Result<(), Box<dyn Error>> {
    //     assert_eq!(part2::run("inputtest.txt")?, 56 * 62);
    //     Ok(())
    // }

    // #[test]
    // fn part2_ans() -> Result<(), Box<dyn Error>> {
    //     assert_eq!(part2::run("input.txt")?, 8250);
    //     Ok(())
    // }
}

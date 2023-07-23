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

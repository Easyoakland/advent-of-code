use crate::data::{Blueprint, Resource};
use advent_lib::parse::yap::{all_consuming, digit1, line_ending, tag, ParseError};
use yap::{one_of, types::StrTokens, IntoTokens, Tokens};

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

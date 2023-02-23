use crate::data::{Monkey, Operation};
use miette::GraphicalReportHandler;
use nom::{
    branch::alt,
    // bytes::complete::tag,
    character::{
        complete::{self as cc, char, digit1, line_ending, one_of},
        streaming::space1,
    },
    combinator::{all_consuming, cut, eof},
    error::{context, ContextError, ParseError},
    multi::{many1, separated_list1},
    sequence::{preceded, terminated, tuple},
    AsChar,
    IResult,
    InputIter,
    InputLength,
    Slice,
};
use nom_locate::LocatedSpan;
use nom_supreme::{
    error::{BaseErrorKind, ErrorTree, GenericErrorTree},
    tag::{complete::tag, TagError},
};
use std::{error::Error, fmt::Debug, ops::RangeFrom, str};

pub type Span<'a> = LocatedSpan<&'a str>;

fn digit1_to_num<'a, I, O, E>(i: I) -> IResult<I, O, E>
where
    O: From<u8>,
    I: InputIter + Slice<RangeFrom<usize>> + InputLength,
    <I as InputIter>::Item: AsChar,
    E: ParseError<I>,
{
    // Extra trait required by below makes it very hard to use in tuple and preceded etc.
    // map_res(digit1, |s: I| s.into().parse::<O>())(i)
    cc::u8(i).map(|(input, out)| (input, out.into()))
}

fn starting_items<'a, E>(i: Span<'a>) -> IResult<Span<'a>, Vec<u8>, E>
where
    E: ParseError<Span<'a>> + TagError<Span<'a>, &'a str>,
{
    let (i, starting_items) = preceded(
        tag("  Starting items: "),
        separated_list1(tag(", "), digit1_to_num),
    )(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, starting_items))
}

fn operation<'a, E>(i: Span<'a>) -> IResult<Span<'a>, Operation, E>
where
    E: ParseError<Span<'a>> + TagError<Span<'a>, &'a str>,
{
    let (i, (op, r)) = tuple((
        preceded(tag("  Operation: new = old "), one_of("*+")),
        preceded(space1, alt((digit1, tag("old")))),
    ))(i)?;
    let (i, _) = line_ending(i)?;
    if let Ok(r) = r.parse() {
        let res = match op {
            '+' => Operation::Add(r),
            '*' => Operation::Mul(r),
            _ => unimplemented!(),
        };
        Ok((i, res))
    } else {
        Err(nom::Err::Failure(E::from_error_kind(
            i,
            nom::error::ErrorKind::Digit,
        )))
    }
}

fn parse_monkey<'a, E>(i: Span<'a>) -> IResult<Span<'a>, Monkey, E>
where
    E: ParseError<Span<'a>> + TagError<Span<'a>, &'a str>,
{
    let (i, _) = tuple((tag("Monkey "), terminated(digit1, char(':'))))(i)?;
    let (i, _) = line_ending(i)?;
    let (i, starting_items) = starting_items(i)?;
    let (i, op) = operation(i)?;
    let (i, test_divisor) = preceded(tag("  Test: divisible by "), digit1_to_num)(i)?;
    let (i, _) = line_ending(i)?;
    let (i, test_true_target) = preceded(tag("    If true: throw to monkey "), digit1_to_num)(i)?;
    let (i, _) = line_ending(i)?;
    let (i, test_false_target) = preceded(tag("    If false: throw to monkey "), digit1_to_num)(i)?;
    let (i, _) = line_ending(i)?;
    Ok((
        i,
        Monkey {
            starting_items,
            op,
            test_divisor,
            test_false_target,
            test_true_target,
        },
    ))
}

// This is not the most efficient but used this to test context for error reporting
fn monkey_list<'a, E>(input: Span<'a>) -> IResult<Span<'a>, Vec<Monkey>, E>
where
    E: ParseError<Span<'a>> + ContextError<Span<'a>> + TagError<Span<'a>, &'a str>,
{
    let (input, mut monkeys) = many1(terminated(cut(parse_monkey), line_ending))(input)?;
    let (input, monkey_final) = context("last monkey", terminated(cut(parse_monkey), eof))(input)?;
    monkeys.push(monkey_final);
    Ok((input, monkeys))
}

fn parse_input<'a, E>(input: Span<'a>) -> IResult<Span<'a>, Vec<Monkey>, E>
where
    E: ParseError<Span<'a>> + ContextError<Span<'a>> + TagError<Span<'a>, &'a str>,
{
    match all_consuming(monkey_list)(input) {
        Ok((input, out)) => Ok((input, out)),
        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => Err(nom::Err::Failure(e)),
        Err(nom::Err::Incomplete(_)) => unimplemented!(),
    }
}

#[derive(thiserror::Error, Debug, miette::Diagnostic)]
#[error("bad input")]
struct BadInput<'a> {
    #[source_code]
    src: &'a str,

    #[label("{kind}")]
    bad_bit: miette::SourceSpan,

    kind: BaseErrorKind<&'a str, Box<dyn std::error::Error + Send + Sync>>,
}

pub fn parse_final(input: Span<'static>) -> Result<Vec<Monkey>, Box<dyn Error>> {
    let monkey_res = parse_input::<ErrorTree<Span>>(input);
    let monkeys_handled_res = match monkey_res {
        Ok(monkeys) => monkeys,
        Err(nom::Err::Error(e) | nom::Err::Failure(e)) => {
            match e {
                GenericErrorTree::Base { location, kind } => {
                    let err_length = match kind {
                        BaseErrorKind::Expected(nom_supreme::error::Expectation::Tag(s)) => {
                            s.len().into()
                        }
                        _ => 0.into(),
                    };
                    let offset = location.location_offset().into();
                    let err = BadInput {
                        src: *input,
                        bad_bit: miette::SourceSpan::new(offset, err_length),
                        kind,
                    };
                    let mut s = String::new();
                    GraphicalReportHandler::new()
                        .render_report(&mut s, &err)
                        .unwrap();
                    println!("{s}");
                }
                x => println!("{x}"),
            }
            return Err("Parsing error")?;
        }
        Err(nom::Err::Incomplete(_)) => unimplemented!(),
    };
    Ok(monkeys_handled_res.1)
}

use std::{
    error::Error,
    fs::{self},
};

mod parse;
use miette::GraphicalReportHandler;
use nom_supreme::{
    error::{BaseErrorKind, ErrorTree, GenericErrorTree},
    final_parser::final_parser,
};
use parse::{parse_input, Span};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part 1 answer: {:#?}", part1::run("input.txt")?);
    Ok(())
}

#[derive(thiserror::Error, Debug, miette::Diagnostic)]
#[error("bad input")]
struct BadInput<'a> {
    #[source_code]
    src: String,

    #[label("{kind}")]
    bad_bit: miette::SourceSpan,

    kind: BaseErrorKind<&'a str, Box<dyn std::error::Error + Send + Sync>>,
}

mod part1 {
    use super::*;
    pub fn run(file: &str) -> Result<u32, Box<dyn Error>> {
        let input_str = fs::read_to_string(file)?;
        let input = Span::new(&input_str);
        let monkey_res = parse::parse_input::<u8, ErrorTree<Span>>(input);
        let monkeys = match monkey_res {
            Ok(monkeys) => monkeys,
            Err(nom::Err::Error(e)) => {
                match e {
                    GenericErrorTree::Base { location, kind } => {
                        let offset = location.location_offset().into();
                        let err = BadInput {
                            src: input_str,
                            bad_bit: miette::SourceSpan::new(offset, 0.into()),
                            kind,
                        };
                        let mut s = String::new();
                        GraphicalReportHandler::new()
                            .render_report(&mut s, &err)
                            .unwrap();
                        println!("{s}");
                    }
                    GenericErrorTree::Stack { .. } => unimplemented!(),
                    GenericErrorTree::Alt(_) => unimplemented!(),
                }
                return Err("Parsing error")?;
            }
            Err(nom::Err::Incomplete(_)) | Err(nom::Err::Failure(_)) => todo!(),
        };
        Ok(0)
    }
}

use anyhow::bail;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char as nom_char, digit1, multispace0},
    combinator::{map, map_res, recognize},
    error::{FromExternalError, ParseError},
    sequence::{delimited, preceded, separated_pair},
    IResult, Parser,
};
use nom_supreme::error::ErrorTree;

use crate::prelude::*;

pub struct Day03;

type DataType = i32;
type OperationParams = (DataType, DataType);

impl Puzzle for Day03 {
    fn new(_ops: &super::RootOpt) -> Box<dyn Puzzle> {
        Box::new(Self)
    }

    fn part_one(&self, _input: &str) -> super::PuzzleResult {
        Ok(extract_mul_chunks(_input)
            .iter()
            .map(|(a, b)| a * b)
            .sum::<DataType>()
            .to_string())
    }

    fn part_two(&self, _input: &str) -> super::PuzzleResult {
        Ok(switchable_extract_mul_chunks(_input)
            .iter()
            .map(|(a, b)| a * b)
            .sum::<DataType>()
            .to_string())
        // todo!("bang")
    }
}

fn extract_two_ints(input: &str) -> Result<OperationParams, anyhow::Error> {
    // find the stuff that's part of the command, and discard the rest
    let mut call_boundary = input.split(")").take(2);
    let params = call_boundary
        .nth(0)
        .expect("No closing paren present")
        .split(",")
        .take(2)
        .filter_map(|p| p.parse::<DataType>().ok())
        .collect_tuple::<OperationParams>();

    // return params.expect("Not a number or wrong number of parameters");
    match params {
        None => bail!("Not a number or wrong number of parameters"),
        Some(result) => Ok(result),
    }
}
fn extract_mul_chunks(input: &str) -> Vec<OperationParams> {
    let mut result: Vec<OperationParams> = Vec::new();
    let chunk_candidates = input.trim().split("mul(").skip(1);
    for chunk in chunk_candidates {
        if let Ok(params) = extract_two_ints(chunk) {
            result.push(params);
        }
    }
    return result;
}
#[test]
fn sample_day03_1() {
    let input = r#"
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
"#;
    println!("{:?}", extract_mul_chunks(input));
}

fn int_parser<'a, E>(input: &'a str) -> IResult<&'a str, DataType, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    map_res(recognize(digit1), str::parse).parse(input)
}
// fn comma_parser<'a, E, F, G, H>(input: &'a str) -> IResult<&'a str, (&'a str, &'a str), E>
fn comma_parser<'a, E>(input: &'a str) -> IResult<&'a str, (&'a str, &'a str), E>
where
    E: ParseError<&'a str>,
    // E: nom::Err<nom::error::Error<&'a str>>,
    // F: Parser<&'a str, &'a str, E>,
    // G: Parser<&'a str, &'a str, E>,
    // H: Parser<&'a str, &'a str, E>,
{
    let multispace = multispace0::<&str, E>;
    // separated_pair::<&'a str, &'a str, char, &'a str, E, F, G, H>(

    separated_pair(
        multispace,
        nom_char::<&str, E>(','),
        // multispace,
        multispace,
    )
    .parse(input)
}
fn params_parser<'a, E>(input: &'a str) -> IResult<&'a str, OperationParams, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    delimited(
        nom_char::<&str, E>('('),
        separated_pair(int_parser, comma_parser, int_parser),
        nom_char::<&str, E>(')'),
    )
    .parse(input)
}

/// Flips a 2-tuple
fn rev2<A, B>((a, b): (A, B)) -> (B, A) {
    (b, a)
}
// Finds junk that doesn't include
fn safe_junk_parser<'a, E>(input: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str>,
{
    // let _ = dbg!(input);
    if input.is_empty() {
        return IResult::Err(nom::Err::Incomplete(nom::Needed::Unknown));
    }

    let min_index = ["do()", "don't()", "mul("]
        .iter()
        .map(|sub| {
            let idx = input.find(sub).map_or(input.len(), |i| i);
            // if idx == 1 {
            //     log::error!("{} {}", idx, input);
            // }
            return idx;
        })
        .filter(|&i| i > 0)
        // .map(|i| {
        //     ///
        //     ///
        //     if i == 1 {
        //         log::error!("{}", i);
        //     }
        //     i
        // })
        .min();

    match min_index {
        Some(the_min) => {
            if the_min == 0 {
                // If it's at the start, it should be handled by something else
                return IResult::Err(nom::Err::Incomplete(nom::Needed::Unknown));
            }
            return IResult::Ok(rev2(input.split_at(the_min)));
        }
        None => {
            // It's not obviously, safely junk, and it's not a known command, chomp one char
            return IResult::Ok(rev2(input.split_at(1)));
        }
    };
}
fn disable_parser<'a, E>(input: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str>,
{
    tag("don't()").parse(input)
}
fn enable_parser<'a, E>(input: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str>,
{
    tag("do()").parse(input)
}
fn mul_parser<'a, E>(input: &'a str) -> IResult<&'a str, OperationParams, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    // preceded(corruption_parser, preceded(tag("mul"), params_parser)).parse(input)
    preceded(tag("mul"), params_parser).parse(input)
}

#[derive(Debug)]
enum Op {
    Junk,
    Disable,
    Enable,
    Mul(OperationParams),
}
fn op_parser<'a, E>(input: &'a str) -> IResult<&'a str, Op, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    if input.is_empty() {
        return IResult::Err(nom::Err::Incomplete(nom::Needed::Unknown));
    }
    // Well formed nodes
    if let Ok(result) = alt((
        map(disable_parser::<E>, |_| Op::Disable),
        map(mul_parser::<E>, Op::Mul),
        map(enable_parser::<E>, |_| Op::Enable),
    ))
    .parse(input)
    {
        return IResult::Ok(result);
    }
    if let Ok((remain, _)) = safe_junk_parser::<E>(input) {
        return IResult::Ok((remain, Op::Junk));
    }
    // It's not obviously, safely junk, and it's not a known command, chomp one char
    return IResult::Ok((&input[1..], Op::Junk));
}
fn ops_parser<'a, E>(input: &'a str) -> IResult<&'a str, Vec<Op>, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError> + std::fmt::Debug,
{
    // fold_many0(op_parser, Vec::new, |mut acc: Vec<_>, item| {
    //     acc.push(item);
    //     acc
    // })
    // .parse(input)
    let mut result: Vec<Op> = Vec::new();
    let mut remain = input;
    loop {
        let state = op_parser::<E>(remain);
        if state.is_ok_and(|(r, op)| {
            remain = r;
            result.push(op);
            true
        }) {
            // nothing else to do
        } else {
            return IResult::Ok((remain, result));
        }
    }
}
#[derive(Debug)]
enum ParserState {
    ScanForMulOrDisable,
    ScanForEnable,
}
fn switchable_extract_mul_chunks(input: &str) -> Vec<OperationParams> {
    let (_, ops) = ops_parser::<ErrorTree<&str>>(input)
        .ok()
        .expect("Couldn't Parse Ops");

    // let (_, ops) = dbg!(tmp);

    let mut remaining_muls: Vec<OperationParams> = Vec::new();
    let mut parser_state = ParserState::ScanForMulOrDisable;
    for op in ops {
        parser_state = match op {
            Op::Junk => parser_state,
            Op::Disable => ParserState::ScanForEnable,
            Op::Enable => ParserState::ScanForMulOrDisable,
            Op::Mul(params) => {
                if matches!(parser_state, ParserState::ScanForMulOrDisable) {
                    remaining_muls.push(params);
                }
                parser_state
            }
        };
    }
    return remaining_muls;
}

#[test]
fn sample_day03_2() {
    let _ = dbg!(tag::<&str, &str, ErrorTree<&str>>("don't()").parse("don't()forget to have fun1"));
    let _ = dbg!(tag::<&str, &str, ErrorTree<&str>>("don't()").parse("expected fail don't()"));
    let _ = dbg!(comma_parser::<ErrorTree<&str>>(" , expected success"));
    let _ = dbg!(params_parser::<ErrorTree<&str>>("(1,2) expected success"));
    let _ = dbg!(params_parser::<ErrorTree<&str>>("(1, expected failure"));
    let _ = dbg!(disable_parser::<ErrorTree<&str>>(
        "don't() expected success"
    ));
    let _ = dbg!(mul_parser::<ErrorTree<&str>>("mul(1,2) expected success"));
    let _ = dbg!(op_parser::<ErrorTree<&str>>("do()expected success"));
    let _ = dbg!(safe_junk_parser::<ErrorTree<&str>>("expected success"));
    let _ = dbg!(safe_junk_parser::<ErrorTree<&str>>("expected do() success"));
    let _ = dbg!(safe_junk_parser::<ErrorTree<&str>>("expected success"));
    let _ = dbg!(ops_parser::<ErrorTree<&str>>("do()don't()mul(1,2)"));
    let _ = dbg!(ops_parser::<ErrorTree<&str>>("do()don't()mul(1,2)mul["));
    let _ = dbg!(ops_parser::<ErrorTree<&str>>("ado()don't()mul(1,2)mul["));
    let _ = dbg!(ops_parser::<ErrorTree<&str>>(
        "ado()mul(1,2don't()mul(1,2)mul["
    ));

    println!(
        "{:?}",
        switchable_extract_mul_chunks("mul(32,64]then(mul(11,8)mul(8,5))")
    );
    let input = r#"
        xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
    "#
    .trim();
    println!(
        "{:?} Result: {:?}",
        switchable_extract_mul_chunks(input),
        switchable_extract_mul_chunks(input)
            .iter()
            .map(|(a, b)| a * b)
            .sum::<DataType>()
            .to_string()
    );

    // println!(
    //     "{:?}",
    //     switchable_extract_mul_chunks("mul(32,64]then(don't()mul(11,8)do()mul(8,5))")
    // );
}

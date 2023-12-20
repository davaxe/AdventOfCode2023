use std::collections::HashMap;

use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::line_ending;
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, separated_pair, terminated};
use nom::IResult;

/// A str slice of direction, e.g. "RLRRL".
#[derive(Debug)]
pub struct Instructions<'a>(pub &'a str);

#[derive(Debug)]
pub struct Node<'a> {
    pub start: &'a str,
    pub left: &'a str,
    pub right: &'a str,
}

fn instructions(input: &str) -> IResult<&str, Instructions> {
    let (input, instr) =
        terminated(take_while1(|c| c == 'L' || c == 'R'), many1(line_ending))(input)?;
    Ok((input, Instructions(instr)))
}

fn node_identifier(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_ascii_uppercase() || c.is_alphanumeric())(input)
}

fn node(input: &str) -> IResult<&str, (&str, Node)> {
    let (input, start) = terminated(node_identifier, tag(" = "))(input)?;
    let (input, (left, right)) = delimited(
        tag("("),
        separated_pair(node_identifier, tag(", "), node_identifier),
        tag(")"),
    )(input)?;
    Ok((input, (start, Node { start, left, right })))
}

fn nodes(input: &str) -> IResult<&str, HashMap<&str, Node>> {
    let (input, nodes) = separated_list1(line_ending, node)(input)?;
    Ok((input, nodes.into_iter().collect()))
}

pub fn map(input: &str) -> IResult<&str, (HashMap<&str, Node>, Instructions)> {
    let (input, instructions) = instructions(input)?;
    let (input, nodes) = nodes(input)?;
    Ok((input, (nodes, instructions)))
}

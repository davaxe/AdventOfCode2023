use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::{complete::tag, complete::take_till},
    character::complete::line_ending,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug)]
pub enum State {
    On,
    Off,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PulseType {
    High,
    Low,
}

#[derive(Debug)]
pub enum ModuleType<'a> {
    FlipFlop(State),
    Conjunction(HashMap<&'a str, PulseType>),
    Broadcast,
}

#[derive(Debug)]
pub struct Module<'a> {
    pub module_type: ModuleType<'a>,
    pub connections: Vec<&'a str>,
    pub identifier: &'a str,
}

/* -------------------------------------------------------------------------- */
/*                              Parsing functions                             */
/* -------------------------------------------------------------------------- */

fn module_type(input: &str) -> IResult<&str, ModuleType> {
    alt((tag("%"), tag("&"), tag("")))(input).map(|(input, t)| match t {
        "%" => (input, ModuleType::FlipFlop(State::Off)),
        "&" => (input, ModuleType::Conjunction(HashMap::new())),
        _ => (input, ModuleType::Broadcast),
    })
}

fn connections(input: &str) -> IResult<&str, Vec<&str>> {
    preceded(
        tag(" -> "),
        separated_list1(tag(", "), take_till(|c: char| !c.is_alphabetic())),
    )(input)
}

fn module(input: &str) -> IResult<&str, (&str, Module)> {
    tuple((module_type, take_till(|c| c == ' '), connections))(input).map(
        |(input, (module_type, identifier, connections))| {
            (
                input,
                (
                    identifier,
                    Module {
                        module_type,
                        identifier,
                        connections,
                    },
                ),
            )
        },
    )
}

pub fn modules(input: &str) -> IResult<&str, HashMap<&str, Module>> {
    separated_list1(line_ending, module)(input)
        .map(|(input, modules)| (input, modules.into_iter().collect()))
}

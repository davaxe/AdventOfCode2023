use nom::bytes::complete::{tag, take_till};
use nom::character::complete::{anychar, digit0, line_ending, space1};
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded, separated_pair};
use nom::IResult;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Instruction<'a> {
    pub direction: Direction,
    pub distance: u32,
    pub color: &'a str,
}

fn direction(input: &str) -> IResult<&str, Direction> {
    anychar(input).map(|(i, dir)| {
        (
            i,
            match dir {
                'U' => Direction::Up,
                'D' => Direction::Down,
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("invalid direction"),
            },
        )
    })
}

fn distance(input: &str) -> IResult<&str, u32> {
    digit0(input).map(|(i, c)| (i, c.parse().expect("distance is a digit")))
}

pub fn color(input: &str) -> IResult<&str, &str> {
    delimited(
        tag("("),
        preceded(tag("#"), take_till(|c| c == ')')),
        tag(")"),
    )(input)
}

pub fn instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, (direction, distance)) = separated_pair(direction, space1, distance)(input)?;
    let (input, color) = preceded(space1, color)(input)?;

    Ok((
        input,
        Instruction {
            direction,
            distance,
            color,
        },
    ))
}

pub fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(line_ending, instruction)(input)
}

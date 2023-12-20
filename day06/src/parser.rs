use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{self, line_ending, space1};
use nom::multi::separated_list1;
use nom::sequence::terminated;
use nom::IResult;

pub fn times(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _) = terminated(tag("Time:"), space1)(input)?;
    terminated(separated_list1(space1, complete::u32), line_ending)(input)
}

pub fn distances(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _) = terminated(tag("Distance:"), space1)(input)?;
    terminated(
        separated_list1(space1, complete::u32),
        alt((line_ending, tag(""))),
    )(input)
}

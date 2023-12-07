use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::terminated,
    IResult,
};

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

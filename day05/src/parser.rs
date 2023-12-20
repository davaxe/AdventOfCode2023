use std::ops::Range;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{self, alpha1, line_ending, space1};
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair, terminated, tuple};
use nom::IResult;

#[derive(Debug)]
pub struct Seed(pub u64);

#[derive(Debug)]
pub struct ConversionMap<'a> {
    pub from: &'a str,
    pub to: &'a str,
    /// Values in the range are offset by the given amount
    pub maps: Vec<(Range<u64>, i128)>,
}

impl ConversionMap<'_> {
    pub fn convert(&self, value: u64) -> u64 {
        for (range, offset) in &self.maps {
            if range.contains(&value) {
                return (value as i128 + offset) as u64;
            }
        }
        value
    }
}

fn seeds(input: &str) -> IResult<&str, Vec<Seed>> {
    let (input, seeds) = preceded(tag("seeds: "), separated_list1(space1, complete::u64))(input)?;
    Ok((input, seeds.into_iter().map(Seed).collect()))
}

fn range(input: &str) -> IResult<&str, (Range<u64>, i128)> {
    let (input, (a, _, b, _, r)) =
        tuple((complete::u64, space1, complete::u64, space1, complete::u64))(input)?;
    let offset = (a as i128) - (b as i128);
    Ok((input, ((b..b + r), offset)))
}

fn map(input: &str) -> IResult<&str, Vec<(Range<u64>, i128)>> {
    let (input, ranges) = separated_list1(line_ending, range)(input)?;
    Ok((input, ranges))
}

fn conversion_map(input: &str) -> IResult<&str, ConversionMap> {
    let (input, (from, to)) = terminated(
        separated_pair(alpha1, tag("-to-"), alpha1),
        tuple((tag(" map:"), line_ending)),
    )(input)?;
    let (input, map) = terminated(map, alt((line_ending, tag(""))))(input)?;
    Ok((
        input,
        ConversionMap {
            from,
            to,
            maps: map,
        },
    ))
}

fn conversion_maps(input: &str) -> IResult<&str, Vec<ConversionMap>> {
    separated_list1(line_ending, conversion_map)(input)
}

pub fn almanac(input: &str) -> IResult<&str, (Vec<Seed>, Vec<ConversionMap>)> {
    let (input, seeds) = terminated(seeds, line_ending)(input)?;
    let (input, maps) = preceded(line_ending, conversion_maps)(input)?;
    Ok((input, (seeds, maps)))
}

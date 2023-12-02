use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha0, line_ending, space0},
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult,
};

#[derive(Debug)]
pub enum Cubes {
    Red(u32),
    Green(u32),
    Blue(u32),
}

impl Cubes {
    fn count(&self) -> (u32, u32, u32) {
        match self {
            Cubes::Red(n) => (*n, 0, 0),
            Cubes::Green(n) => (0, *n, 0),
            Cubes::Blue(n) => (0, 0, *n),
        }
    }
}

#[derive(Debug)]
pub struct CubeSet {
    pub cubes: Vec<Cubes>,
}

impl CubeSet {
    /// Returns the maximum number of cubes of each color in this set
    pub fn max_count(&self) -> (u32, u32, u32) {
        self.cubes
            .iter()
            .map(|c| c.count())
            .fold((0, 0, 0), |(r, g, b), (r1, g1, b1)| {
                (u32::max(r, r1), u32::max(g, g1), u32::max(b, b1))
            })
    }
}

#[derive(Debug)]
pub struct Game {
    pub id: u32,
    pub sets: Vec<CubeSet>,
}

fn cube(input: &str) -> IResult<&str, Cubes> {
    let (input, n) = preceded(space0, complete::u32)(input)?;
    // Parse color and convert to enum
    let (input, color) = preceded(space0, alpha0)(input).map(|(input, color)| match color {
        "blue" => (input, Cubes::Blue(n)),
        "red" => (input, Cubes::Red(n)),
        "green" => (input, Cubes::Green(n)),
        _ => panic!("Invalid color"),
    })?;
    Ok((input, color))
}

fn set(input: &str) -> IResult<&str, CubeSet> {
    let (input, cubes) = separated_list1(tag(","), cube)(input)?;
    Ok((input, CubeSet { cubes }))
}

fn game(input: &str) -> IResult<&str, Game> {
    let (input, game) = delimited(tag("Game "), complete::u32, tag(":"))(input)?;
    let (input, sets) = separated_list1(tag(";"), set)(input)?;
    Ok((input, Game { id: game, sets }))
}

/// Parse a list of games, where each line is a game of the form given in the problem
/// description
pub fn games(input: &str) -> IResult<&str, Vec<Game>> {
    separated_list1(line_ending, game)(input)
}

use crate::parser;

pub use parser::{CubeSet, Cubes, Game};

pub fn task(input: &str) -> Option<String> {
    let (_, games) = parser::games(input).ok()?;
    // Filter out games that are not valid, then sum the ids of the remaining games
    Some(
        games
            .iter()
            .filter(|g| valid_game(g))
            .map(|g| g.id)
            .sum::<u32>()
            .to_string(),
    )
}

fn valid_game(game: &Game) -> bool {
    // Constants from the problem description
    const MAX_RED_CUBES: u32 = 12;
    const MAX_GREEN_CUBES: u32 = 13;
    const MAX_BLUE_CUBES: u32 = 14;

    let (red, green, blue) = game
        .sets
        .iter()
        .map(|s| s.max_count())
        // Get the maximum number of cubes of each color used in this game
        .fold((0, 0, 0), |(r, g, b), (r1, g1, b1)| {
            (u32::max(r, r1), u32::max(g, g1), u32::max(b, b1))
        });
    red <= MAX_RED_CUBES && green <= MAX_GREEN_CUBES && blue <= MAX_BLUE_CUBES
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../part1-example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "8");
    }
}

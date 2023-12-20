use nom::bytes::complete::take_till;
use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use nom::IResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    RoundRock,
    StaticObstacle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PossibleMovementResult {
    Possible((usize, usize)),
    Blocked,
    OutOfBounds,
}

#[derive(Debug, Clone)]
pub struct Board {
    /// Collection of (x, y) coordinates of round rocks
    round_rocks: Vec<(usize, usize)>,
    /// Tiles
    tiles: Vec<Vec<Tile>>,
}

impl Board {
    /// Move all round rocks in the given direction until no more movement is
    /// possible
    pub fn move_round_rocks(&mut self, direction: Direction) -> u32 {
        let mut steps = 0;
        while self.move_round_rocks_step(direction) > 0 {
            steps += 1;
        }
        steps
    }

    /// Move all round rocks in the given direction one step
    fn move_round_rocks_step(&mut self, direction: Direction) -> u32 {
        let mut changes = Vec::new();

        self.round_rocks.iter().enumerate().for_each(|(i, (x, y))| {
            if let PossibleMovementResult::Possible((xn, yn)) =
                self.possible_movement(*x, *y, direction)
            {
                changes.push((i, (xn, yn)));
            }
        });

        changes.iter().for_each(|(i, (xn, yn))| {
            let (x_old, y_old) = self.round_rocks[*i];
            self.tiles[y_old][x_old] = Tile::Empty;
            self.tiles[*yn][*xn] = Tile::RoundRock;
            self.round_rocks[*i] = (*xn, *yn);
        });

        changes.len() as _
    }

    /// Calculate the total load of the board in its current state, as defined
    /// by the problem
    pub fn total_load(&self) -> u32 {
        let height = self.tiles.len();
        self.round_rocks
            .iter()
            .fold(0, |acc, (_x, y)| acc + (height - y) as u32)
    }

    /// Calculate the possible movement of a rock at position (x, y) in the
    /// given  direction.
    fn possible_movement(
        &self,
        x: usize,
        y: usize,
        direction: Direction,
    ) -> PossibleMovementResult {
        let width = self.tiles[0].len() - 1;
        let height = self.tiles.len() - 1;

        let (xn, yn) = match direction {
            Direction::North if y > 0 => (x, y - 1),
            Direction::South if y < height => (x, y + 1),
            Direction::East if x < width => (x + 1, y),
            Direction::West if x > 0 => (x - 1, y),
            _ => return PossibleMovementResult::OutOfBounds,
        };

        if let Tile::Empty = self.tiles[yn][xn] {
            PossibleMovementResult::Possible((xn, yn))
        } else {
            PossibleMovementResult::Blocked
        }
    }
}

pub fn board(input: &str) -> IResult<&str, Board> {
    let (input, rows) = separated_list1(line_ending, row)(input)?;

    let mut round_rocks = Vec::new();

    rows.iter()
        .enumerate()
        .for_each(|(r, (_, round_column_i))| {
            round_column_i.iter().for_each(|c| {
                round_rocks.push((*c, r));
            });
        });

    Ok((
        input,
        Board {
            round_rocks,
            tiles: rows.into_iter().map(|(tiles, _)| tiles).collect(),
        },
    ))
}

fn row(input: &str) -> IResult<&str, (Vec<Tile>, Vec<usize>)> {
    let mut round_rock_column_indices = Vec::new();

    let (input, tiles) = take_till(|c| c == '\r')(input)?;
    let tiles: Vec<Tile> = tiles
        .char_indices()
        .map(|(i, c)| match c {
            'O' => {
                round_rock_column_indices.push(i);
                Tile::RoundRock
            }
            '#' => Tile::StaticObstacle,
            '.' => Tile::Empty,
            _ => panic!("Invalid character in board"),
        })
        .collect();

    Ok((input, (tiles, round_rock_column_indices)))
}

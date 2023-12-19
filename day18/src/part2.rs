use crate::parser::{self, Direction};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Rotation {
    Clockwise,
    CounterClockwise,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Side {
    x: i128,
    y: i128,
    direction: Direction,
    length: u32,
}

pub fn task(input: &str) -> Option<String> {
    let (_, instructions) = parser::instructions(input).ok()?;

    // Convert to correct instructions as described in part 2
    let mut x = 0;
    let mut y = 0;
    let mut sides = instructions
        .into_iter()
        .filter_map(|instruction| {
            let distance: u32 = u32::from_str_radix(&instruction.color[..5], 16).ok()?;
            Some(match &instruction.color[5..] {
                "0" => {
                    let side = Side {
                        x,
                        y,
                        direction: Direction::Right,
                        length: distance,
                    };
                    x += distance as i128;
                    side
                }
                "1" => {
                    let side = Side {
                        x,
                        y,
                        direction: Direction::Down,
                        length: distance,
                    };
                    y += distance as i128;
                    side
                }
                "2" => {
                    let side = Side {
                        x,
                        y,
                        direction: Direction::Left,
                        length: distance,
                    };
                    x -= distance as i128;
                    side
                }
                "3" => {
                    let side = Side {
                        x,
                        y,
                        direction: Direction::Up,
                        length: distance,
                    };
                    y -= distance as i128;
                    side
                }
                _ => return None,
            })
        })
        .collect::<Vec<_>>();

    let last_direction = sides.last()?.direction;
    let rotation = match last_direction {
        Direction::Up | Direction::Right => Rotation::Clockwise,
        Direction::Down | Direction::Left => Rotation::CounterClockwise,
    };

    if rotation == Rotation::Clockwise {
        sides.reverse();
    }

    // Shoelace Algorithm
    let mut a = sides
        .windows(2)
        .map(|win| {
            let a = win[0];
            let b = win[1];
            (a.x, a.y, b.x, b.y)
        })
        .fold(0i128, |acc, (x1, y1, x2, y2)| acc + (x1 * y2) - (y1 * x2));

    a += sides.last()?.x * sides.first()?.y;
    a -= sides.last()?.y * sides.first()?.x;

    let edge = sides.iter().fold(0u32, |acc, side| acc + side.length) as i128;

    // Not sure why one needs to be added here, but it works
    let area = (a.abs() + edge) / 2 + 1;
    Some(area.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "952408144115");
    }
}

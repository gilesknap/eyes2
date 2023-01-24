// simple standalone utility functions

use crate::types::{Direction, Position};
use rand::Rng;

// Pick a random direction (we could shorten this a little using
// https://enodev.fr/posts/rusticity-convert-an-integer-to-an-enum.html
// but copilot wrote this more me and it reads nicely)
pub fn random_direction() -> Direction {
    let mut rng = rand::thread_rng();
    let direction = rng.gen_range(0..8);
    match direction {
        0 => Direction::North,
        1 => Direction::NorthEast,
        2 => Direction::East,
        3 => Direction::SouthEast,
        4 => Direction::South,
        5 => Direction::SouthWest,
        6 => Direction::West,
        7 => Direction::NorthWest,
        _ => panic!("bad direction"),
    }
}

// copilot wrote this one too
pub fn move_pos(position: Position, direction: Direction) -> Position {
    match direction {
        Direction::North => Position {
            x: position.x,
            y: position.y - 1,
        },
        Direction::NorthEast => Position {
            x: position.x + 1,
            y: position.y - 1,
        },
        Direction::East => Position {
            x: position.x + 1,
            y: position.y,
        },
        Direction::SouthEast => Position {
            x: position.x + 1,
            y: position.y + 1,
        },
        Direction::South => Position {
            x: position.x,
            y: position.y + 1,
        },
        Direction::SouthWest => Position {
            x: position.x - 1,
            y: position.y + 1,
        },
        Direction::West => Position {
            x: position.x - 1,
            y: position.y,
        },
        Direction::NorthWest => Position {
            x: position.x - 1,
            y: position.y - 1,
        },
    }
}

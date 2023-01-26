//! simple standalone utility functions

use crate::types::{Direction, Position};
use rand::Rng;

/// Pick a random direction (we could shorten this a little using
/// [int to enum](https://enodev.fr/posts/rusticity-convert-an-integer-to-an-enum.html)
/// but copilot wrote this more me and it reads nicely)
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

/// the underflow checking is rather verbose, can this be simplified?
/// TODO also add max for x and y
/// TODO come up with a neater way to do this (can't I make the enum Variants
/// hold values for the x and y offsets?)
pub fn move_pos(pos: Position, direction: Direction, size: u16) -> Position {
    let max = size - 1;
    match direction {
        Direction::North => Position {
            x: pos.x,
            y: if pos.y > 0 { pos.y - 1 } else { 0 },
        },
        Direction::NorthEast => Position {
            x: if pos.x < max { pos.x + 1 } else { max },
            y: if pos.y > 0 { pos.y - 1 } else { 0 },
        },
        Direction::East => Position {
            x: if pos.x < max { pos.x + 1 } else { max },
            y: pos.y,
        },
        Direction::SouthEast => Position {
            x: if pos.x < max { pos.x + 1 } else { max },
            y: if pos.y < max { pos.y + 1 } else { max },
        },
        Direction::South => Position {
            x: pos.x,
            y: if pos.y < max { pos.y + 1 } else { max },
        },
        Direction::SouthWest => Position {
            x: if pos.x > 0 { pos.x - 1 } else { 0 },
            y: if pos.y < max { pos.y + 1 } else { max },
        },
        Direction::West => Position {
            x: if pos.x > 0 { pos.x - 1 } else { 0 },
            y: pos.y,
        },
        Direction::NorthWest => Position {
            x: if pos.x > 0 { pos.x - 1 } else { 0 },
            y: if pos.y > 0 { pos.y - 1 } else { 0 },
        },
    }
}

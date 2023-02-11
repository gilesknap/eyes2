//! simple standalone utility functions

use direction::{Coord, Direction};
use fastrand::Rng as FastRng;

// return a coordinate that is one step in the given direction from the given coordinate
pub fn move_pos(pos: Coord, dir: Direction, size: u16) -> Coord {
    let max = size - 1;
    let mut new_pos = pos + dir.coord();

    new_pos.x = new_pos.x.clamp(0, max as i32);
    new_pos.y = new_pos.y.clamp(0, max as i32);

    new_pos
}

pub fn rotate_direction(dir: Direction) -> Direction {
    match dir {
        Direction::North => Direction::NorthEast,
        Direction::NorthEast => Direction::East,
        Direction::East => Direction::SouthEast,
        Direction::SouthEast => Direction::South,
        Direction::South => Direction::SouthWest,
        Direction::SouthWest => Direction::West,
        Direction::West => Direction::NorthWest,
        Direction::NorthWest => Direction::North,
    }
}

pub fn random_direction(rng: &FastRng) -> Direction {
    // TODO is this the fastest way to do this?
    match rng.u8(0..8) {
        0 => Direction::North,
        1 => Direction::NorthEast,
        2 => Direction::East,
        3 => Direction::SouthEast,
        4 => Direction::South,
        5 => Direction::SouthWest,
        6 => Direction::West,
        7 => Direction::NorthWest,
        _ => unreachable!(),
    }
}

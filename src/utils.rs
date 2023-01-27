//! simple standalone utility functions

use direction::{Coord, Direction};

// return a coordinate that is one step in the given direction from the given coordinate
pub fn move_pos(pos: Coord, dir: Direction, size: i32) -> Coord {
    let max = size - 1;
    let mut new_pos = pos + dir.coord();

    new_pos.x = new_pos.x.clamp(0, max);
    new_pos.y = new_pos.y.clamp(0, max);

    new_pos
}

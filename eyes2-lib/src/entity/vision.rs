// define a type and associated functions to represent the vision of an entity
// this initial implementation is for a creature with 8 directions of vision
// (N, NE, E, SE, S, SW, W, NW) with a single cell visible in each direction
use direction::{Coord, Direction, Directions};

use crate::{Cell, WorldGrid};

pub type Vision = [Cell; 8];

// TODO I'd like to make these functions methods on the Vision type but
// because its a native type I need something called an extension trait

// create a vision array for the given coordinate in a world grid
pub fn look_world(coord: Coord, grid: &WorldGrid) -> Vision {
    let mut vision = [Cell::Empty; 8];
    for (i, direction) in Directions::into_iter(Directions).enumerate() {
        vision[i] = grid.get_cell(coord + direction.coord());
    }
    vision
}

pub fn get_vision_in_direction(vision: Vision, direction: &Direction) -> Cell {
    vision[*direction as usize]
}

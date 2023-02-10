use direction::Coord;

use crate::entity::entity::Cell;

// the representation of the world cells plus some metadata
// used to pass information to the renderer thread
pub struct WorldGrid {
    // the grid of cells
    grid: Vec<Cell>,
    // number of grass blocks in the world
    pub grass_count: u64,
    // the dimensions of the (square) grid
    pub size: u16,
}

impl WorldGrid {
    pub fn new(size: u16) -> WorldGrid {
        // create a square 2d vector of empty cells
        let grid = vec![Cell::Empty; size.pow(2) as usize];

        WorldGrid {
            grid,
            grass_count: 0,
            size,
        }
    }

    /// read a cell from the grid - used for rendering the world
    #[inline(always)]
    pub fn get_cell(&self, position: Coord) -> Cell {
        // TODO Currently using Copy to return this - maybe should switch to
        // using a Box? Then could remove the Copy trait from Cell
        self.grid[(position.x + position.y * self.size as i32) as usize]
    }

    #[inline(always)]
    pub fn set_cell(&mut self, position: Coord, value: Cell) {
        self.grid[(position.x + position.y * self.size as i32) as usize] = value;
    }

    pub fn add_grass(&mut self, coord: Coord) {
        match self.get_cell(coord) {
            Cell::Empty => {
                self.set_cell(coord, Cell::Grass);
                self.grass_count += 1;
            }
            _ => {}
        }
    }

    pub fn remove_grass(&mut self, coord: Coord) {
        match self.get_cell(coord) {
            Cell::Grass => {
                self.set_cell(coord, Cell::Empty);
                self.grass_count -= 1;
            }
            _ => {}
        }
    }
}

use direction::Coord;

use crate::entity::entity::Cell;

// the representation of the world cells plus some metadata
// used to pass information to the renderer thread
#[derive(Clone, Debug)]
pub struct WorldGrid {
    // the grid of cells
    grid: Vec<Cell>,
    // the dimensions of the (square) grid
    size: u16,
    // number of grass blocks in the world
    grass_count: u64,
    // ticks since the world started
    pub ticks: u64,
}

impl WorldGrid {
    pub fn new(size: u16) -> WorldGrid {
        // create a square 2d vector of empty cells
        let grid = vec![Cell::Empty; size.pow(2) as usize];

        WorldGrid {
            grid,
            size,
            grass_count: 0,
            ticks: 0,
        }
    }

    #[inline(always)]
    pub fn get_size(&self) -> u16 {
        self.size
    }

    #[inline(always)]
    pub fn grass_count(&self) -> usize {
        self.grass_count as usize
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

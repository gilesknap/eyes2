use crate::entity;
use direction;

// the representation of the world cells plus some metadata
// used to pass information to the renderer thread
#[derive(Clone, Debug)]
pub struct WorldGrid {
    // the grid of cells
    grid: Vec<entity::Cell>,
    // the dimensions of the (square) grid
    size: u16,
    // number of grass blocks in the world
    grass_count: u64,
    // the interval between grass growth events
    pub grass_rate: u64,
    // number of grass blocks in the world
    pub creature_count: u64,
    // simulation speed
    pub speed: u64,
    // ticks since the world started
    pub ticks: u64,
    // number of world restarts
    pub restarts: u64,
}

impl WorldGrid {
    pub fn new(size: u16, grass_rate: u64, speed: u64, restarts: u64) -> WorldGrid {
        // create a square 2d vector of empty cells
        let grid = vec![entity::entity::Cell::Empty; size.pow(2) as usize];

        WorldGrid {
            grid,
            size,
            grass_count: 0,
            grass_rate,
            speed,
            creature_count: 0,
            ticks: 0,
            restarts,
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

    #[inline(always)]
    pub fn creature_count(&self) -> usize {
        self.creature_count as usize
    }

    pub fn increment_grass_rate(&mut self, up: bool) {
        if up {
            self.grass_rate += 1;
        } else {
            self.grass_rate -= 1;
        }
        self.grass_rate = self.grass_rate.clamp(1, 100)
    }

    pub fn increment_speed(&mut self, up: bool) {
        if up {
            self.speed += 1;
        } else {
            self.speed -= 1;
        }
        self.speed = self.speed.clamp(1, 10)
    }

    /// read a cell from the grid - used for rendering the world
    #[inline(always)]
    pub fn get_cell(&self, position: direction::Coord) -> entity::Cell {
        // TODO Currently using Copy to return this - maybe should switch to
        // using a Box? Then could remove the Copy trait from entity::Cell
        self.grid[(position.x + position.y * self.size as i32) as usize]
    }

    #[inline(always)]
    pub fn set_cell(&mut self, position: direction::Coord, value: entity::Cell) {
        self.grid[(position.x + position.y * self.size as i32) as usize] = value;
    }

    pub fn add_grass(&mut self, coord: direction::Coord) {
        if let entity::Cell::Empty = self.get_cell(coord) {
            self.set_cell(coord, entity::Cell::Grass);
            self.grass_count += 1;
        }
    }

    pub fn remove_grass(&mut self, coord: direction::Coord) {
        if let entity::Cell::Grass = self.get_cell(coord) {
            self.set_cell(coord, entity::Cell::Empty);
            self.grass_count -= 1;
        }
    }
}

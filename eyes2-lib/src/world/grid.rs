use chrono::{DateTime, Utc};

use direction;
use serde::{Serialize, Deserialize};

// the representation of the world cells plus some metadata
// used to pass information to the renderer thread
#[derive(Clone, Debug, Serialize,  Deserialize)]
pub struct WorldGrid {
    // the grid of cells
    #[serde(skip)]
    grid: Vec<Cell>,
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
    // start time of current restart
    #[serde(skip)]
    pub start_time: DateTime<Utc>,
    // next unique id to assign to an Entity
    pub next_id: u64,
}

// represent the contents of a single cell in the world
#[derive(Debug, Copy, Clone)]
pub enum Cell {
    // the cell is empty
    Empty,

    // the cell is occupied by a Creature (with a unique number)
    Entity(u64, char),

    // the cell is occupied by a block of grass
    Grass,
}

impl WorldGrid {
    pub fn new(size: u16, grass_rate: u64, speed: u64, restarts: u64) -> WorldGrid {
        // create a square 2d vector of empty cells
        let grid = vec![Cell::Empty; size.pow(2) as usize];

        WorldGrid {
            grid,
            size,
            grass_count: 0,
            grass_rate,
            speed,
            creature_count: 0,
            ticks: 0,
            restarts,
            start_time: Utc::now(),
            next_id: 0,
        }
    }

    // restore correct size of the grid after loading from a file
    pub fn expand (&mut self, size: u16) {
        self.grid = vec![Cell::Empty; size.pow(2) as usize];
    }

    pub fn get_size(&self) -> u16 {
        self.size
    }

    pub fn grass_count(&self) -> usize {
        self.grass_count as usize
    }

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

    pub fn get_cell(&self, position: direction::Coord) -> Cell {
        // Note this is a Copy return but its just a little enum, right?
        self.grid[(position.x + position.y * self.size as i32) as usize]
    }

    pub fn set_cell(&mut self, position: direction::Coord, value: Cell) {
        self.grid[(position.x + position.y * self.size as i32) as usize] = value;
    }

    pub fn add_grass(&mut self, coord: direction::Coord) {
        if let Cell::Empty = self.get_cell(coord) {
            self.set_cell(coord, Cell::Grass);
            self.grass_count += 1;
        }
    }

    pub fn remove_grass(&mut self, coord: direction::Coord) {
        if let Cell::Grass = self.get_cell(coord) {
            self.set_cell(coord, Cell::Empty);
            self.grass_count -= 1;
        }
    }
}

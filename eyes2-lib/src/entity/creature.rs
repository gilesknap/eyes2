use super::{Update, UpdateQueue};
use crate::Settings;
use direction::Coord;
use fastrand::Rng as FastRng;

use crate::utils::{move_pos, random_direction};

pub struct Creature {
    id: u64,
    coord: Coord,
    energy: i32,
    config: Settings,
    rng: FastRng,
    herbivore: bool,
}

// The representation of a creature in the world
impl Creature {
    pub fn new(coord: Coord, config: Settings) -> Creature {
        let rng = FastRng::new();
        let (b, e) = config.creature_initial_energy;
        let energy = rng.i32(b..e);

        Creature {
            id: 0,
            coord,
            energy,
            rng,
            config,
            herbivore: true,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn coord(&self) -> Coord {
        self.coord
    }

    pub fn move_to(&mut self, pos: Coord) {
        self.coord = pos;
    }

    pub fn set_id(&mut self, id: u64) {
        // id is immutable once set
        if self.id == 0 {
            self.id = id;
        }
    }

    pub fn eat(&mut self, amount: i32) {
        self.energy += amount;
    }

    pub fn tick(&mut self, queue: &mut UpdateQueue) {
        self.energy -= self.config.creature_idle_energy;

        if self.energy <= 0 {
            queue.push(Update::RemoveEntity(self.id, self.coord()));
        } else if self.energy >= self.config.creature_reproduction_energy {
            self.reproduce(queue);
        } else if self.rng.f32() <= self.config.creature_move_rate {
            let direction = random_direction(&self.rng);
            let new_pos = move_pos(self.coord, direction, self.config.size);

            self.energy -= self.config.creature_move_energy;
            queue.push(Update::MoveEntity(self.id(), self.coord(), new_pos));
        }
    }
}

// private instance methods
impl Creature {
    fn reproduce(&mut self, queue: &mut UpdateQueue) {
        let mut child = Creature::new(self.coord, self.config);
        self.energy /= 2;
        child.energy = self.energy;
        // child is spawned to the left unless we are against the left wall
        if self.coord.x == 0 {
            child.coord.x += 1;
        } else {
            child.coord.x -= 1
        }
        queue.push(Update::AddEntity(child));
    }
}

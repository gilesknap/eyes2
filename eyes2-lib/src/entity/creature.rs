/// The representation of a creature in the world
///
/// This module implements the generic behaviour of a creature and enforces
/// the rules of the world. The rules are:-
///
/// 1. A creature can move one cell in any of the 8 directions (including diagonals)
/// 2. A herbivore can eat grass if it is in the same cell as the grass
/// 3. A carnivore can eat another creature if it is in the same cell as the other creature
/// 4. A creature can reproduce if it has enough energy
/// 5. A creature dies if it has no energy
/// 6. A creature can request the value adjacent cells (i.e. the vision in 'eyes)
///
/// The rules are implemented in the tick() method which is called once per tick
/// of the world. Global settings control the energy costs and rewards of each action.
///
/// The specific behaviour of an individual is determined by its genotype.
///
use super::{new_genotype, Genotype};
use super::{Update, UpdateQueue};
use crate::Settings;
use direction::Coord;
use fastrand::Rng as FastRng;

use crate::utils::{move_pos, random_direction};

pub struct Creature {
    // the unique id of the creature used to identify it in the world
    id: u64,
    // the position of the creature in the world for reverse lookup
    coord: Coord,
    // the amount of energy the creature has
    energy: i32,
    // global settings for the world which include generic creature settings
    config: Settings,
    // each creature has its own random number generator
    rng: FastRng,
    // the world rules are different for herbivores and carnivores
    _herbivore: bool,
    // the genotype of the creature which determines its behaviour
    _genotype: Box<dyn Genotype>,
}

// The representation of a creature in the world
impl Creature {
    pub fn new(coord: Coord, config: Settings) -> Creature {
        let rng = FastRng::new();
        let (b, e) = config.creature_initial_energy;
        let energy = rng.i32(b..e);

        // YAY! polymorphism in rust!
        let genotype = new_genotype("random").expect("unknown genotype requested");

        Creature {
            id: 0,
            coord,
            energy,
            rng,
            config,
            _herbivore: true,
            _genotype: genotype,
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

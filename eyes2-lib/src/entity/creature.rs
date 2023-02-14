//! The representation of a creature in the world
//!
//! This module implements the generic behaviour of a creature and enforces
//! the rules of the world. The rules are:-
//!
//! 1. A creature can move one cell in any of the 8 directions (including diagonals)
//! 2. A herbivore can eat grass if it is in the same cell as the grass
//! 3. A carnivore can eat another creature if it is in the same cell as the other creature
//! 4. A creature can reproduce if it has enough energy
//! 5. A creature dies if it has no energy
//! 6. A creature can request the value adjacent cells (i.e. the vision in 'eyes)
//!
//! The rules are implemented in the tick() method which is called once per tick
//! of the world. Global settings control the energy costs and rewards of each action.
//!
//! The specific behaviour of an individual is determined by its genotype.
//!
//! genotypes must implement the Genotype trait and are registered in the
//! new_genotype() function.
//!
//! genotypes call back into the creature to perform actions such as moving
//! 'looking' via the GenotypeCallback trait.
//!
use crate::utils::move_pos;
use std::rc::Rc;
use std::sync::mpsc;

use super::genotype::genotype::GenotypeActions;
use super::Update;

use super::Genotype;
use crate::Settings;
use direction::{Coord, Direction};
use fastrand::Rng as FastRng;

pub struct Creature {
    // the unique id of the creature used to identify it in the world
    id: u64,
    // the position of the creature in the world for reverse lookup
    coord: Coord,
    // the amount of energy the creature has
    energy: i32,
    // global settings for the world which include generic creature settings
    config: Settings,
    // transmitter to send updates to the world
    tx: Rc<mpsc::Sender<Update>>,
    // the world rules are different for herbivores and carnivores
    _herbivore: bool,
    // the genotype of the creature which determines its behaviour
    genotype: Box<dyn Genotype>,
    // the sigil used to represent the creature in the world
    sigil: char,
}

// The representation of a creature in the world
impl Creature {
    pub fn new(
        genotype: Box<dyn Genotype>,
        coord: Coord,
        config: Settings,
        tx: Rc<mpsc::Sender<Update>>,
    ) -> Creature {
        let (b, e) = config.creature_initial_energy;

        // TODO maybe pass a pre-created rng around to avoid creating a new one each time
        let rng = FastRng::new();
        let energy = rng.i32(b..e);
        let sigil = genotype.get_sigil();

        Creature {
            id: 0,
            coord,
            energy,
            config,
            tx,
            _herbivore: true,
            genotype,
            sigil: sigil,
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
        self.genotype.set_energy(self.energy);
    }

    pub fn tick(&mut self) {
        self.energy -= self.config.creature_idle_energy;

        // check for death
        if self.energy <= 0 {
            self.tx
                .send(Update::RemoveEntity(self.id, self.coord()))
                .expect("failed to send remove entity");
            return;
        }

        // call the genotype specific tick method
        match self.genotype.tick() {
            GenotypeActions::Move(direction) => self.move_dir(direction),
            GenotypeActions::Reproduce(genotype) => self.reproduce(genotype),
            GenotypeActions::Look(direction) => self.look(direction),
            GenotypeActions::None => {}
        }
    }

    pub fn get_sigil(&self) -> char {
        self.sigil
    }
}

// private instance methods
impl Creature {
    fn reproduce(&mut self, genotype: Box<dyn Genotype>) {
        // TODO need to get child genotype into the new child
        let mut child = Creature::new(genotype, self.coord, self.config.clone(), self.tx.clone());
        self.energy /= 2;
        child.energy = self.energy;
        // child is spawned to the left unless we are against the left wall
        if self.coord.x == 0 {
            child.coord.x += 1;
        } else {
            child.coord.x -= 1
        }
        self.tx
            .send(Update::AddEntity(child))
            .expect("creature reproduce failed");
    }

    fn move_dir(&mut self, direction: Direction) {
        let new_pos = move_pos(self.coord, direction, self.config.size);

        self.energy -= self.config.creature_move_energy;
        self.tx
            .send(Update::MoveEntity(self.id(), self.coord(), new_pos))
            .expect("failed to send move entity");
    }

    fn look(&mut self, _direction: Direction) {
        // TODO
    }
}

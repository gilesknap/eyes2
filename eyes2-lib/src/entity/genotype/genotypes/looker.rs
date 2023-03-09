//! Implement the Looker genotype, which is a creature that moves in one direction
//! but turns when it sees food to the left or right.

use super::{Genotype, GenotypeActions};
use crate::{
    entity::{get_vision_in_direction, Vision},
    Cell, Settings,
};
use direction::Direction;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct LookerGenotype {
    #[serde(skip)]
    config: Settings,
    energy: i32,
    direction: Direction,
    next_action: LookerAction,
    next_tick: u64,
    ticks_per_move: u64,
    reproduction_scale: i32,
}

// Looker alternates between moving and looking each tick
#[derive(Serialize, Deserialize, Clone)]
enum LookerAction {
    Move,
    Look,
}

const MOVE_TICKS: u64 = 500;
const REPRODUCTION_SCALE: i32 = 10;

#[typetag::serde(name = "looker_genotype")]
impl Genotype for LookerGenotype {
    fn tick(&mut self) -> GenotypeActions {
        if self.energy >= self.config.creature_reproduction_energy * self.reproduction_scale {
            return GenotypeActions::Reproduce(Box::new(self.reproduce()));
        }

        match self.next_tick {
            0 => {
                self.next_tick = self.ticks_per_move;

                match self.next_action {
                    LookerAction::Move => {
                        self.next_action = LookerAction::Look;
                        return GenotypeActions::Move(self.direction);
                    }
                    LookerAction::Look => {
                        self.next_action = LookerAction::Move;
                        return GenotypeActions::Look;
                    }
                }
            }
            _ => {
                self.next_tick -= 1;
            }
        }

        GenotypeActions::None
    }

    fn vision(&mut self, vision: Vision) {
        match get_vision_in_direction(vision, &self.direction) {
            // grass ahead, keep going immediately
            Cell::Grass => self.next_tick = 0,
            // obstacle ahead, turn around
            Cell::Wall | Cell::Entity(_, _) => self.direction = self.direction.opposite(),
            // otherwise, look for grass to the left or right
            Cell::Empty => {
                for turn in [&self.direction.left90(), &self.direction.right90()].iter() {
                    if let Cell::Grass = get_vision_in_direction(vision, turn) {
                        self.direction = *turn.clone();
                        self.next_tick = 0;
                    }
                }
            }
        }
    }

    fn set_energy(&mut self, energy: i32) {
        self.energy = energy;
    }

    fn get_sigil(&self) -> char {
        'L'
    }
}

impl LookerGenotype {
    pub fn new(config: Settings) -> LookerGenotype {
        LookerGenotype {
            config,
            energy: 0,
            direction: Direction::North,
            next_action: LookerAction::Look,
            next_tick: MOVE_TICKS,
            ticks_per_move: MOVE_TICKS,
            reproduction_scale: REPRODUCTION_SCALE,
        }
    }

    pub fn reproduce(&mut self) -> Self {
        self.energy -= self.config.creature_reproduction_energy;
        self.clone()
    }
}

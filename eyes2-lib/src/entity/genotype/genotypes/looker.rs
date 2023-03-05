//! Implement the Looker genotype, which is a creature that moves in one direction
//! but turns when it sees food to the left or right.

use super::{Genotype, GenotypeActions};
use crate::{entity::Vision, Cell, Settings};
use direction::Direction;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct LookerGenotype {
    #[serde(skip)]
    config: Settings,
    energy: i32,
    direction: Direction,
    next_action: LookerAction,
    next_move_ticks: u64,
}

// Looker alternates between moving and looking each tick
#[derive(Serialize, Deserialize, Clone)]
enum LookerAction {
    Move,
    Look,
}

const MOVE_TICKS: u64 = 500;

#[typetag::serde(name = "looker_genotype")]
impl Genotype for LookerGenotype {
    fn tick(&mut self) -> GenotypeActions {
        if self.energy >= self.config.creature_reproduction_energy {
            return GenotypeActions::Reproduce(Box::new(self.reproduce()));
        }

        match self.next_action {
            LookerAction::Move => {
                self.next_action = LookerAction::Look;
                match self.next_move_ticks {
                    0 => {
                        self.next_move_ticks = MOVE_TICKS;
                        return GenotypeActions::Move(self.direction);
                    }
                    _ => {
                        self.next_move_ticks -= 1;
                    }
                }
            }
            LookerAction::Look => {
                self.next_action = LookerAction::Move;
                return GenotypeActions::Look;
            }
        }
        GenotypeActions::None
    }

    fn vision(&mut self, vision: Vision) {
        for (direction, cell) in vision {
            match cell {
                Cell::Grass => {
                    self.direction = direction;
                    self.next_move_ticks = 0;
                    break;
                }
                Cell::Empty => {}
                _ => {
                    self.direction = self.direction.opposite();
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
            next_move_ticks: MOVE_TICKS,
        }
    }

    pub fn reproduce(&mut self) -> Self {
        self.energy -= self.config.creature_reproduction_energy;
        self.clone()
    }
}

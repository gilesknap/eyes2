use direction::Direction;
use dyn_clone::{clone_trait_object, DynClone};

use crate::{Cell, Settings};

#[derive(Debug)]
pub enum BadGenomeError {
    InvalidGenome,
}

// Every creature has a Genotype which defines their behaviour. It is
// expected that the Genotype will be defined by a genome, and that the
// genome (with mutations as appropriate) will be passed to the
// descendant creatures.
#[typetag::serde(tag = "type")]
pub trait Genotype: DynClone {
    // execute the next instruction of your Genomic code
    fn tick(&mut self) -> GenotypeActions;

    // change your internal energy level (this is for reference only as
    // the canonical energy level in in Creature itself)
    fn set_energy(&mut self, energy: i32);

    // return the sigil used to represent this creature in the world
    fn get_sigil(&self) -> char {
        'D'
    }

    // A callback from the world to return the view of the world from
    // the last Look(Direction) action. The value is a 1D array of 4
    // Cells. With the nearest cell the first in the array.
    fn vision(&self, _direction: Direction, _value: [Cell; 4]) {}
}
clone_trait_object!(Genotype);

// The genotype's tick method returns one of these actions. Creature
// will pass the request on to the world which will verify the
// action is valid and then update the world state accordingly.
pub enum GenotypeActions {
    Reproduce(Box<dyn Genotype>),
    Move(Direction),
    Look(Direction),
    None,
}

// For each new Genotype defined the developer must add an arm to this
// genotype constructor function. This constructor provides a polymorphic
// interface to the Genotype trait.
pub fn new_genotype(which: &str, config: Settings) -> Result<Box<dyn Genotype>, BadGenomeError> {
    let genotype: Box<dyn Genotype> = match which {
        "giles" => Box::new(super::genotypes::giles::GilesGenotype::new(config)),
        "noop" => Box::new(super::genotypes::noop::NoopGenotype::new(config)),
        "random" => Box::new(super::genotypes::random::RandomGenotype::new(config)),
        _ => return Err(BadGenomeError::InvalidGenome),
    };
    Ok(genotype)
}

use direction::Direction;

use super::genotypes::giles::GilesGenotype;
use super::genotypes::random::RandomGenotype;
use crate::Settings;
// use crate::entity::Creature;
// use crate::Settings;

#[derive(Debug)]
pub enum BadGenomeError {
    InvalidGenome,
}

// Every creature has a Genotype which defines their behaviour. It is
// expected that the Genotype will be defined by a genome, and that the
// genome (with mutations as appropriate) will be passed to the
// descendant creatures.
pub trait Genotype {
    // // constructor
    // fn new(creature: Creature, config: Settings) -> Self;

    // execute the next instruction of your Genomic code
    fn tick(&mut self);
    fn set_energy(&mut self, energy: i32);
}

pub trait GenotypeCallback {
    // request a move in the given direction
    fn move_dir(&mut self, direction: Direction);
    fn reproduce(&mut self, child: Box<dyn Genotype>);
    fn look(&mut self, direction: Direction);
}

// For each new Genotype defined the developer must add an arm to this
// genotype constructor function. This constructor provides a polymorphic
// interface to the Genotype trait.
pub fn new_genotype(which: &str, config: Settings) -> Result<Box<dyn Genotype>, BadGenomeError> {
    let genotype: Box<dyn Genotype> = match which {
        "giles" => Box::new(GilesGenotype { energy: 0 }),
        "random" => Box::new(RandomGenotype::new(config)),
        _ => return Err(BadGenomeError::InvalidGenome),
    };
    Ok(genotype)
}

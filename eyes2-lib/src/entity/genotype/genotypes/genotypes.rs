//! A module for defining multiple genotypes
//!
//! To add a new genotype, create a new module in the genotypes directory and
//! implement the Genotype trait. Then update the Genotypes struct in this
//! module to include the new genotype.
//!
//!
use serde::{Deserialize, Serialize};

use super::giles::GilesGenotype;
use super::noop::NoopGenotype;
use super::random::RandomGenotype;
use crate::settings::Settings;

#[derive(Serialize, Deserialize)]
pub enum Genotypes {
    Giles(GilesGenotype),
    Noop(NoopGenotype),
    Random(RandomGenotype),
}

#[derive(Debug)]
pub enum BadGenomeError {
    InvalidGenome,
}

impl Genotypes {
    pub fn new(which: &str, config: Settings) -> Result<Genotypes, BadGenomeError> {
        match which {
            "giles" => Ok(Genotypes::Giles(GilesGenotype::new(config))),
            "noop" => Ok(Genotypes::Noop(NoopGenotype::new(config))),
            "random" => Ok(Genotypes::Random(RandomGenotype::new(config))),
            _ => Err(BadGenomeError::InvalidGenome),
        }
    }
}

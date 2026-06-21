use direction::Direction;
use dyn_clone::{clone_trait_object, DynClone};

use crate::{entity::Vision, Settings};

#[derive(Debug)]
pub enum BadGenomeError {
    InvalidGenome,
}

/// One line of a genotype's program listing, for the inspector.
#[derive(Debug, Clone)]
pub struct InspectLine {
    /// the address of the line within the genome
    pub addr: usize,
    /// the rendered instruction text
    pub text: String,
}

/// A genotype-agnostic snapshot of a creature's "brain" for the TUI inspector.
///
/// Genotypes that have inspectable internal state (such as the `giles` byte-code
/// VM) return one of these from [`Genotype::inspect`]; the GUI renders it
/// without needing to know anything about the specific genotype.
#[derive(Debug, Clone)]
pub struct GenotypeInspect {
    /// a short name for the kind of genotype, e.g. "giles"
    pub kind: &'static str,
    /// labelled register / state values, e.g. ("R", "0x2a")
    pub state: Vec<(String, String)>,
    /// the program listing (may be empty for genotypes without code)
    pub listing: Vec<InspectLine>,
    /// the index into `listing` of the instruction about to execute, if any
    pub active: Option<usize>,
}

// Every creature has a Genotype which defines their behaviour. It is
// expected that the Genotype will be defined by a genome, and that the
// genome (with mutations as appropriate) will be passed to the
// descendant creatures.
//
// `Send` is a supertrait so that `Box<dyn Genotype>` (and therefore a whole
// `Creature`) can be moved between threads. This is what lets the simulation
// run the per-creature "think" phase in parallel across all cores - see
// DESIGN_MULTITHREAD.md. Every genotype only holds `Send` data, so this costs
// nothing.
#[typetag::serde(tag = "type")]
pub trait Genotype: DynClone + Send {
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
    fn vision(&mut self, _vision: Vision) {}

    // Return a snapshot of internal state for the TUI inspector, or None for
    // genotypes that have nothing interesting to show.
    fn inspect(&self) -> Option<GenotypeInspect> {
        None
    }
}
clone_trait_object!(Genotype);

// The genotype's tick method returns one of these actions. Creature
// will pass the request on to the world which will verify the
// action is valid and then update the world state accordingly.
pub enum GenotypeActions {
    Reproduce(Box<dyn Genotype>),
    Move(Direction),
    Look,
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
        "looker" => Box::new(super::genotypes::looker::LookerGenotype::new(config)),
        _ => return Err(BadGenomeError::InvalidGenome),
    };
    Ok(genotype)
}

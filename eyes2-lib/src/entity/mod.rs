pub mod creature;
mod genotype;
pub mod update;

pub use self::creature::Creature;
pub use self::genotype::genotype::Genotype;
pub use self::update::{Update, UpdateQueue};

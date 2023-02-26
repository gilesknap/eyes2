pub mod creature;
mod genotype;
pub mod update;

pub use self::creature::Creature;
pub use self::genotype::genotype::{new_genotype, Genotype, GenotypeActions};
pub use self::genotype::genotypes::genotypes::Genotypes;
pub use self::update::{Update, UpdateQueue};

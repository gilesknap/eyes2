pub mod creature;
pub mod genotype;
pub mod update;
pub mod vision;

pub use self::creature::{Creature, CreatureInspect};
pub use self::genotype::genotype::{new_genotype, Genotype, GenotypeInspect, InspectLine};
pub use self::update::{Update, UpdateQueue};
pub use self::vision::{get_vision_in_direction, look_world, Vision};

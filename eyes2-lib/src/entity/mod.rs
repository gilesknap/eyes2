pub mod creature;
mod genotype;
pub mod update;
pub mod vision;

pub use self::creature::Creature;
pub use self::genotype::genotype::{new_genotype, Genotype};
pub use self::update::{Update, UpdateQueue};
pub use self::vision::{get_vision_in_direction, look_world, Vision};

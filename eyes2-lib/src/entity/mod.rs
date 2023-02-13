pub mod creature;
// TODO public while we work on allowing multiple genotypes
pub mod genotype;
pub mod update;

pub use self::creature::Creature;
pub use self::genotype::genotype::Genotype;
pub use self::update::Update;

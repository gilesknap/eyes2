use crate::entity::update::UpdateQueue;
// use crate::entity::Creature;
// use crate::Settings;

pub trait Genotype {
    // // constructor
    // fn new(creature: Creature, config: Settings) -> Self;

    // execute the next instruction of your Genomic code
    fn tick(&mut self, queue: &mut UpdateQueue);
}

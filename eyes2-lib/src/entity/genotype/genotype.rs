pub trait Genotype
where
    Self: Sized,
{
    // constructor
    fn new(entity: Entity, config: Settings) -> Self;

    // execute the next instruction of your Genomic code
    fn tick(&mut self, queue: &mut UpdateQueue);
}

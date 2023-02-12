pub trait Genotype
where
    Self: Sized,
{
    // constructor
    fn new(coord: Coord, config: Settings) -> Self;

    // execute the next instruction of your Genomic code
    fn tick(&mut self, queue: &mut UpdateQueue);
}

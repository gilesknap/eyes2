
#[derive(Debug)]
pub struct GilesGenotype {
    pub energy: i32,       // energy level
    ip: u16,               // instruction pointer
    a: u16,                // accumulator
    i: [u16; 5],           // registers
    breed_rate: u32,       // rate at which the creature breeds
    mutation_rate: u32,    // rate at which the creature mutates
    genome: [u16; GENOME], // the genome i.e. the instructions to be executed
}

impl GilesGenotype {
    pub fn new(energy: i32) -> GilesGenotype {
        GilesGenotype {
            energy,
            ip: 0,
            a: 0,
            i: [0; 5],
            breed_rate: 0,
            mutation_rate: 0,
            genome: GilesGenotype::randomize(),
        }
    }

    pub fn randomize() -> [u16; GENOME] {
        let mut genome = [0; GENOME];
        for i in 0..genome.len() {
            genome[i] = fastrand::u16(..);
        }
        genome
    }

    // totally dummy instruction set for now
    pub fn tick(&mut self) {
        self.ip = (self.ip + 1) % (GENOME as u16);
        let instruction = self.genome[self.ip as usize];
        match instruction {
            0 => self.a = self.i[0],
            1 => self.a = self.breed_rate as u16,
            2 => self.a = self.mutation_rate as u16,
            _ => (),
        }
    }
use serde::{Deserialize, Serialize};

use crate::Settings;

use super::{Genotype, GenotypeActions};

const _GENOME: usize = 1000;

#[derive(Serialize, Deserialize)]
pub struct NoopGenotype {
    #[serde(skip)]
    config: Settings,
    energy: i32,
}

#[typetag::serde(name = "noop_genotype")]
impl Genotype for NoopGenotype {
    fn tick(&mut self) -> GenotypeActions {
        let _dummy = self.config.size;
        GenotypeActions::None
    }

    fn set_energy(&mut self, energy: i32) {
        self.energy = energy;
    }

    fn get_sigil(&self) -> char {
        'N'
    }
}

impl NoopGenotype {
    pub fn new(config: Settings) -> NoopGenotype {
        NoopGenotype { config, energy: 0 }
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    // world size in cells (square)
    pub size: u16,
    // number of grass blocks to add to the world
    pub grass_count: u16,
    // number of creatures to add to the world
    pub creature_count: u16,
    // number of ticks between grass growth
    pub grass_interval: u16,
    // energy gained from eating grass
    pub grass_energy: u32,
}

const DEFAULT_SETTINGS: Settings = Settings {
    size: 40,
    grass_count: 10,
    creature_count: 25,
    grass_interval: 5000,
    grass_energy: 1000,
};

impl Settings {
    pub fn load() -> Settings {
        confy::load("eyes2", None).unwrap()
    }

    pub fn save(&self, settings: Settings) {
        confy::store("eyes2", None, settings).unwrap();
    }
}

impl ::std::default::Default for Settings {
    fn default() -> Settings {
        DEFAULT_SETTINGS
    }
}

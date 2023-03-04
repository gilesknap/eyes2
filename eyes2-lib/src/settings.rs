use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    // world size in cells (square)
    pub size: u16,
    // number of grass blocks to add to the world
    pub grass_count: u16,
    // energy gained from eating grass
    pub grass_energy: i32,
    // range of energy for new creatures
    pub creature_initial_energy: (i32, i32),
    // energy level for reproduction
    pub creature_reproduction_energy: i32,
    // energy lost from moving
    pub creature_move_energy: i32,
    // energy gained from idling
    pub creature_idle_energy: i32,
    // Speed of Creature movement chance of moving per tick
    // Only used for Random movement mode (not Genome based movement control)
    pub creature_move_rate: f32, // MAX 1.0

    // these are initial values for settings that change during runtime

    // speed of the simulation
    pub speed: u64,
    // number of ticks between grass growth
    pub grass_rate: u64,

    // number of creatures of each genome type to add to the world
    // see eyes2-lib/src/entity/genotype/genotype.rs new_genotype()
    // for the list of valid genome types
    // TODO: the String should be an enum?
    pub creatures: Vec<(String, u16)>,
}

impl Settings {
    pub fn load() -> Settings {
        let mut settings: Settings = confy::load("eyes2", None).unwrap();
        settings.size = settings.size.clamp(10, 200);
        settings.grass_count = settings.grass_count.clamp(0, settings.size.pow(2));
        for creature in settings.creatures.iter_mut() {
            creature.1 = creature.1.clamp(0, settings.size.pow(2));
        }
        settings.creature_move_rate = settings.creature_move_rate.clamp(0.0, 1.0);

        settings.speed = settings.speed.clamp(1, 10);
        settings.grass_rate = settings.grass_rate.clamp(1, 100);

        settings
    }

    pub fn save(&self, settings: Settings) {
        confy::store("eyes2", None, settings).unwrap();
    }

    pub fn reset() -> Settings {
        confy::store("eyes2", None, Settings::default()).unwrap();
        Settings::default()
    }
}

impl ::std::default::Default for Settings {
    fn default() -> Settings {
        Settings {
            size: 40,
            grass_count: 1000,
            grass_energy: 1000,
            creature_initial_energy: (10000, 20000),
            creature_reproduction_energy: 100000,
            creature_move_energy: 10,
            creature_idle_energy: 1,
            creature_move_rate: 0.005,

            speed: 9,
            grass_rate: 85,

            creatures: vec![
                ("random".to_string(), 10),
                ("noop".to_string(), 10),
                ("giles".to_string(), 10),
            ],
        }
    }
}

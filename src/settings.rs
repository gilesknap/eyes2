pub struct Settings {
    pub world_size: u16,
    pub grass_count: u16,
    pub creature_count: u16,
}

const DEFAULT_SETTINGS: Settings = Settings {
    world_size: 40,
    grass_count: 10,
    creature_count: 25,
};

pub static SETTINGS: Settings = DEFAULT_SETTINGS;

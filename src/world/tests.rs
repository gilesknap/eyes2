use direction::Coord;

use crate::settings::Settings;

// use all items from the parent module
use super::World;

const CONFIG: Settings = Settings {
    size: 40,
    grass_count: 10,
    creature_count: 25,
    grass_interval: 5000,
    max_grass_per_interval: 500,
    grass_energy: 1000,
    creature_move_energy: 100,
    creature_idle_energy: 1,
    creature_move_rate: 0.01,
};

#[test]
fn check_add_creature() {
    let mut world = World::new(CONFIG);

    world
        .creatures
        .add_new_entity(Coord { x: 0, y: 0 })
        .unwrap();
    assert_eq!(world.creature_count(), 1);
}

#[test]
fn check_populate() {
    let mut world = World::new(CONFIG);
    world.populate();

    assert_eq!(world.grass_count(), CONFIG.grass_count as usize);
    assert_eq!(world.creature_count(), CONFIG.creature_count as usize);

    world.grass.remove_entity(&0);
    world.creatures.remove_entity(&0);

    assert_eq!(world.grass_count(), CONFIG.grass_count as usize - 1);
    assert_eq!(world.creature_count(), CONFIG.creature_count as usize - 1);
}

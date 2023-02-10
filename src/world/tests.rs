use crate::settings::Settings;

// use all items from the parent module
use super::types::World;

fn get_config() -> Settings {
    Settings {
        size: 40,
        grass_count: 10,
        creature_count: 25,
        grass_interval: 5000,
        max_grass_per_interval: 500,
        grass_energy: 1000,
        creature_move_energy: 100,
        creature_idle_energy: 1,
        creature_move_rate: 0.01,
        ..Settings::default()
    }
}

#[test]
fn check_add_creature() {
    let mut _world = World::new(get_config());

    // world
    //     .creatures
    //     .add_new_entity(Coord { x: 0, y: 0 })
    //     .unwrap();
    // assert_eq!(world.creature_count(), 1);
}

#[test]
fn check_populate() {
    let config = get_config();
    let mut world = World::new(config);
    world.populate();

    assert!(world.grass_count() <= config.grass_count as usize);
    assert!(world.creature_count() <= config.creature_count as usize);

    let creature_count = world.creature_count();

    world.creatures.remove(&0);

    assert_eq!(world.creature_count(), creature_count - 1 as usize);
}

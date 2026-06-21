use super::*;

use crate::settings::Settings;

fn get_config() -> Settings {
    Settings {
        size: 40,
        grass_count: 10,
        grass_rate: 5000,
        grass_energy: 1000,
        creature_move_energy: 100,
        creature_idle_energy: 1,
        creature_move_rate: 0.01,
        ..Settings::default()
    }
}

#[test]
fn check_add_creature() {
    let mut _world = World::new(get_config(), 0);

    assert_eq!(_world.creature_count(), 0);

    let _genotype = new_genotype("noop", _world.config.clone());
    let _creature = Creature::new(
        Box::new(_genotype).unwrap(),
        Coord { x: 1, y: 1 },
        _world.config.clone(),
    );
    // queue an AddEntity intent and resolve it, exactly as a tick would
    _world.queue_update(Update::AddEntity(_creature));
    _world.apply_updates();

    assert_eq!(_world.creature_count(), 1);
}

#[test]
fn check_populate() {
    let config = get_config();
    let mut world = World::new(config.clone(), 0);
    world.populate();

    assert!(world.grid.grass_count() <= config.grass_count as usize);

    let _creature_count = world.creature_count();
    // remove the first creature in the store by its id
    let first_id = world.creatures[0].id();
    world.remove_creature(first_id);

    assert_eq!(world.creature_count(), _creature_count - 1 as u64);
}

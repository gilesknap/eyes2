use crate::types::Position;

// use all items from the parent module
use super::World;

#[test]
fn check_add_creature() {
    let mut world = World::new(10);

    world.creatures.add_entity(Position { x: 0, y: 0 }).unwrap();
    assert_eq!(world.creature_count(), 1);
}

#[test]
fn check_populate() {
    let mut world = World::new(10);
    world.populate(10, 10);

    assert_eq!(world.grass_count(), 10);
    assert_eq!(world.creature_count(), 10);

    world.grass.remove_entity(&0);
    world.creatures.remove_entity(&0);

    assert_eq!(world.grass_count(), 9);
    assert_eq!(world.creature_count(), 9);
}

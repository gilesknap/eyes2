// use all items from the parent module
use super::World;

#[test]
fn check_add_creature() {
    // let mut world = World::new(10);
}

#[test]
fn check_populate() {
    let mut world = World::new(10);
    world.populate(10, 10);

    assert_eq!(world.grass_count(), 10);
    assert_eq!(world.creature_count(), 10);

    world
        .grass
        .remove_entity(0)
        .expect("Failed to remove grass 0");
    world
        .creatures
        .remove_entity(0)
        .expect("Failed to remove creature 0");

    assert_eq!(world.grass_count(), 9);
    assert_eq!(world.creature_count(), 9);
}

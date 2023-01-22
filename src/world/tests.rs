// use all items from the parent module
use super::{Cell, Creature, Position, World};

#[test]
fn check_add_creature() {
    let mut world = World::new(10);
    let position = Position { x: 0, y: 0 };
    let creature = Creature::new(position, 1000);

    let creature_num = creature.num;
    assert_eq!(world.add_creature(creature), Ok(()));

    // previous creature was moved into the world so we can't use it again
    let creature = Creature::new(position, 1000);
    assert_eq!(world.add_creature(creature), Err(()));
    assert!(world.creatures.len() == 1);

    // verify lookup cells via position in the world
    assert!(matches!(
        world.get_cell(Position { x: 1, y: 1 }),
        Cell::Empty
    ));
    assert!(matches!(world.get_cell(position), Cell::Creature(_)));

    for (key, value) in &world.creatures {
        println!("{}: {:?}", key, value);
    }

    // verify lookup creature via its number
    let creature_pos = world
        .creature(creature_num)
        .expect("can't find creature {}")
        .position;

    assert_eq!(creature_pos, position);
}

#[test]
fn check_populate() {
    let mut world = World::new(10);
    world.populate(10, 10, 1000);

    assert_eq!(world.grass_count(), 10);
    assert_eq!(world.creature_count(), 10);

    let creature = Creature::new(Position { x: 1, y: 1 }, 1000);

    // won't remove creature because it's not in the world
    assert_eq!(world.remove_creature(&creature), Err(()));
    assert_eq!(world.creature_count(), 10);

    // TODO this is where things fall down - I can't do the following
    // because:
    //   world.creatures borrows the world immutable
    //   then world.remove_creature borrows the world mutable
    // borrower does not allow this.
    //
    // while world.creature_count() > 0 {
    //     let creature = world.creatures.values().next().unwrap().clone();
    //     world.remove_creature(&creature);
    // }
}

// use all items from the parent module
use super::*;

#[test]
fn check_add_creature() {
    let mut world = World::new(10);
    let position = Position { x: 0, y: 0 };
    let creature = Creature::new(position, 1000);

    let creature_num = creature.num;
    world.add_creature(creature);

    // previous creature was moved into the world so we can't use it again
    let creature = Creature::new(position, 1000);
    assert_eq!(world.add_creature(creature), false);
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
        .creatures
        .get(&creature_num)
        .expect("can't find creature {}")
        .position;

    assert_eq!(creature_pos, position);
}

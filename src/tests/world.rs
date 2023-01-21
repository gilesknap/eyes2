#[cfg(test)] // TODO repeating this implies that I have a bad structure???
use crate::creature::Creature;
#[cfg(test)]
use crate::types::Position;
#[cfg(test)]
use crate::world::Cell;
#[cfg(test)]
use crate::world::World;

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

    let _x = creature.num;

    // verify lookup cells via position in the world
    assert!(matches!(
        world.get_cell(Position { x: 1, y: 1 }),
        Cell::Empty
    ));
    assert!(matches!(world.get_cell(position), Cell::Creature(_)));

    for (key, value) in &world.creatures {
        println!("{}: {:?}", key, value);
    }

    let &creature = world
        .creatures
        .get(&creature_num)
        .expect("can't find creature {}");
    assert_eq!(creature.position, position);
}

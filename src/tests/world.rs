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
    let creature = Creature::new(Position { x: 0, y: 0 }, 1000);
    world.add_creature(creature);
    // previous creature was moved into the world so we can't use it again
    let creature = Creature::new(Position { x: 0, y: 0 }, 1000);
    assert_eq!(world.add_creature(creature), false);
    assert!(world.creatures.len() == 1);

    assert!(matches!(
        world.get_cell(Position { x: 1, y: 1 }),
        Cell::Empty
    ));
    assert!(matches!(
        world.get_cell(Position { x: 0, y: 0 }),
        Cell::Creature(_)
    ));
}

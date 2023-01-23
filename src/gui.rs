use crate::entity::Cell;
use crate::types::Position;
use crate::world::World;

// TODO rudimentary rendering of the world as a placeholder
pub fn render(world: &World) {
    println!();
    println!("Rendering World of size: {}", world.get_size());

    for y in 0..world.get_size() {
        for x in 0..world.get_size() {
            match world.get_cell(Position { x, y }) {
                Cell::Empty => print!(" "),
                Cell::Grass(_) => print!("*"),
                Cell::Creature(_) => print!("o"),
            };
        }
        println!();
    }
}

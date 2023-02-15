use crate::world::World;
use std::fs::File;

pub fn save_world(world: &World) {
    let world_data = world.serialize();
    let file = File::create("world.yaml").unwrap();
    serde_yaml::to_writer(file, &world_data).unwrap();
}

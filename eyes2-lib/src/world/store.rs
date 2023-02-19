use super::*;
use ::direction::Coord;
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::fs::File;

pub fn save_world(world: &World) {
    let file = File::create("world.yaml").unwrap();
    serde_yaml::to_writer(file, world).unwrap();
}

impl Serialize for World {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("World", 1)?;
        s.serialize_field("size", &self.get_size())?;
        s.serialize_field("next_id", &self.next_id)?;
        s.serialize_field("grass_rate", &self.grid.grass_rate)?;
        s.serialize_field("speed", &self.grid.speed)?;
        s.serialize_field("ticks", &self.grid.ticks)?;

        // serialize the grid by type of cell - don't record empty cells
        let mut grasses: Vec<Coord> = Vec::new();
        let mut creatures: Vec<(Coord, u64)> = Vec::new();
        for x in 0..self.config.size as i32 {
            for y in 0..self.config.size as i32 {
                let coord = Coord { x, y };
                let cell = self.grid.get_cell(coord);
                match cell {
                    Cell::Entity(id, _) => {
                        creatures.push((coord, id));
                    }
                    Cell::Grass => {
                        grasses.push(coord);
                    }
                    _ => {}
                }
            }
        }

        s.serialize_field("creatures", &creatures)?;
        s.serialize_field("grasses", &grasses)?;

        s.end()
    }
}

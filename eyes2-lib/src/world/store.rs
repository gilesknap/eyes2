//! Save and load the world to/from a YAML file

use super::*;
use crate::entity::Creature;
use crate::settings::Settings;
use direction::Coord;
use serde::de;
use serde::de::{Deserializer, MapAccess, Visitor};
use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::File;

const FIELDS: &'static [&'static str] = &["grid", "config", "creatures", "grasses"];

#[derive(Deserialize, Serialize)]
struct CreatureSer {
    coord: Coord,
    creature: Creature,
}

pub fn save_world(world: &World) {
    let file = File::create("world.yaml").unwrap();
    serde_yaml::to_writer(file, world).unwrap();
}

pub fn load_world() -> World {
    let file = File::open("world.yaml").unwrap();
    let world: World = serde_yaml::from_reader(file).unwrap();
    world
}

impl Serialize for World {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("World", 1)?;

        // serialize the grid into a more human readable format -
        // two vectors of coordinates, one for grass, one for creatures
        let mut grasses: Vec<Coord> = Vec::new();
        let mut creatures: Vec<CreatureSer> = Vec::new();
        for x in 0..self.config.size as i32 {
            for y in 0..self.config.size as i32 {
                let coord = Coord { x, y };
                let cell = self.grid.get_cell(coord);
                match cell {
                    Cell::Entity(id, _) => {
                        let creature = self.creatures.get(&id).unwrap().clone();
                        creatures.push(CreatureSer { coord, creature });
                    }
                    Cell::Grass => {
                        grasses.push(coord);
                    }
                    _ => {}
                }
            }
        }

        s.serialize_field(FIELDS[0], &self.grid)?;
        s.serialize_field(FIELDS[1], &self.config)?;
        s.serialize_field(FIELDS[2], &creatures)?;
        s.serialize_field(FIELDS[3], &grasses)?;

        s.end()
    }
}

impl<'de> Deserialize<'de> for World {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            Grid,
            Config,
            Creatures,
            Grasses,
        }

        struct WorldSerVisitor;

        impl<'de> Visitor<'de> for WorldSerVisitor {
            type Value = World;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct World")
            }

            fn visit_map<V>(self, mut map: V) -> Result<World, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut config: Option<Settings> = None;
                let mut grid: Option<WorldGrid> = None;
                let mut creatures: Option<Vec<CreatureSer>> = None;
                let mut grasses: Option<Vec<Coord>> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Grid => grid = Some(map.next_value()?),
                        Field::Config => config = Some(map.next_value()?),
                        Field::Creatures => creatures = Some(map.next_value()?),
                        Field::Grasses => grasses = Some(map.next_value()?),
                    }
                }
                let config = config.ok_or_else(|| de::Error::missing_field("config"))?;
                let mut grid = grid.ok_or_else(|| de::Error::missing_field("grid"))?;
                grid.expand(config.size);

                let mut world: World = World::load(config, grid);
                for creature_coord in creatures.unwrap() {
                    let mut creature = creature_coord.creature.clone();
                    creature.move_to(creature_coord.coord);
                    creature.set_tx(world.tx.clone());
                    world.tx.send(Update::AddEntity(creature)).unwrap();
                }
                for grass_coord in grasses.unwrap() {
                    world.grid.add_grass(grass_coord);
                }
                world.apply_updates();
                Ok(world)
            }
        }

        deserializer.deserialize_struct("world", FIELDS, WorldSerVisitor)
    }
}

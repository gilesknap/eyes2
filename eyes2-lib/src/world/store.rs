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


const FIELDS: &'static [&'static str] = &[
    "config",
    "next_id",
    "grass_rate",
    "speed",
    "ticks",
    "creatures",
    "grasses",
];

#[derive(Deserialize, Serialize)]
struct CreatureCoord {
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
        let field_num = 0;
        s.serialize_field(FIELDS[field_num], &self.config)?;
        s.serialize_field("next_id", &self.next_id)?;
        s.serialize_field("grass_rate", &self.grid.grass_rate)?;
        s.serialize_field("speed", &self.grid.speed)?;
        s.serialize_field("ticks", &self.grid.ticks)?;

        // serialize the grid by type of cell - don't record empty cells
        let mut grasses: Vec<Coord> = Vec::new();
        let mut creatures: Vec<CreatureCoord> = Vec::new();
        for x in 0..self.config.size as i32 {
            for y in 0..self.config.size as i32 {
                let coord = Coord { x, y };
                let cell = self.grid.get_cell(coord);
                match cell {
                    Cell::Entity(id, _) => {
                        let creature = self.creatures.get(&id).unwrap().clone();
                        creatures.push(CreatureCoord { coord, creature });
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

impl<'de> Deserialize<'de> for World {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            Config,
            NextId,
            GrassRate,
            Speed,
            Ticks,
            Creatures,
            Grasses,
        }

        struct WorldVisitor;

        impl<'de> Visitor<'de> for WorldVisitor {
            type Value = World;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct World")
            }

            fn visit_map<V>(self, mut map: V) -> Result<World, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut config: Option<Settings> = None;
                let mut next_id: Option<u64> = None;
                let mut grass_rate: Option<u64> = None;
                let mut speed: Option<u64> = None;
                let mut ticks: Option<u64> = None;
                let mut creatures: Option<Vec<CreatureCoord>> = None;
                let mut _grasses: Option<Vec<Coord>> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Config => config = Some(map.next_value()?),
                        Field::NextId => next_id = Some(map.next_value()?),
                        Field::GrassRate => grass_rate = Some(map.next_value()?),
                        Field::Speed => speed = Some(map.next_value()?),
                        Field::Ticks => ticks = Some(map.next_value()?),
                        Field::Creatures => creatures = Some(map.next_value()?),
                        Field::Grasses => _grasses = Some(map.next_value()?),
                    }
                }
                let config = config.ok_or_else(|| de::Error::missing_field("config"))?;
                let next_id = next_id.ok_or_else(|| de::Error::missing_field("nextid"))?;
                let grass_rate = grass_rate.ok_or_else(|| de::Error::missing_field("grassrate"))?;
                let speed = speed.ok_or_else(|| de::Error::missing_field("speed"))?;
                let ticks = ticks.ok_or_else(|| de::Error::missing_field("ticks"))?;

                let grid = WorldGrid::new(config.size, grass_rate, speed, ticks);
                let mut world = World::load(config, grid, next_id);
                for creature_coord in creatures.unwrap() {
                    let mut creature = creature_coord.creature.clone();
                    creature.move_to(creature_coord.coord);
                    creature.set_tx(world.tx.clone());
                    world.tx.send(Update::AddEntity(creature)).unwrap();
                }
                for grass_coord in _grasses.unwrap() {
                    world.grid.add_grass(grass_coord);
                }
                world.apply_updates();

                Ok(world)
            }
        }

        deserializer.deserialize_struct("world", FIELDS, WorldVisitor)
    }
}

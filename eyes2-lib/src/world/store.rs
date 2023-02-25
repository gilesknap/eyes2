use super::*;
use ::direction::Coord;
use serde::de;
use serde::de::{Deserializer, MapAccess, Visitor};
use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::File;

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
        s.serialize_field("config", &self.config)?;
        s.serialize_field("nextid", &self.next_id)?;
        s.serialize_field("grassrate", &self.grid.grass_rate)?;
        s.serialize_field("speed", &self.grid.speed)?;
        s.serialize_field("ticks", &self.grid.ticks)?;

        // serialize the grid by type of cell - don't record empty cells
        let mut grasses: Vec<Coord> = Vec::new();
        let mut creatures: Vec<(Coord, &Creature)> = Vec::new();
        for x in 0..self.config.size as i32 {
            for y in 0..self.config.size as i32 {
                let coord = Coord { x, y };
                let cell = self.grid.get_cell(coord);
                match cell {
                    Cell::Entity(id, _) => {
                        let creature = self.creatures.get(&id).unwrap().clone();
                        creatures.push((coord, creature));
                    }
                    Cell::Grass => {
                        grasses.push(coord);
                    }
                    _ => {}
                }
            }
        }

        s.serialize_field("creaturecount", &creatures.len())?;
        s.serialize_field("creatures", &creatures)?;
        s.serialize_field("grasscount", &grasses.len())?;
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
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Config,
            NextId,
            GrassRate,
            Speed,
            Ticks,
            CreatureCount,
            Creatures,
            GrassCount,
            Grasses,
        }

        const FIELDS: &'static [&'static str] = &[
            "config",
            "nextid",
            "grassrate",
            "speed",
            "ticks",
            "creaturecount",
            "creatures",
            "grasscount",
            "grasses",
        ];

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
                let mut config = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Config => {
                            if config.is_some() {
                                return Err(de::Error::duplicate_field("config"));
                            }
                            config = Some(map.next_value()?);
                        }
                        _ => {}
                    }
                }
                let config = config.ok_or_else(|| de::Error::missing_field("secs"))?;
                Ok(World::new(config, 0))
            }
        }

        deserializer.deserialize_struct("world", FIELDS, WorldVisitor)
    }
}

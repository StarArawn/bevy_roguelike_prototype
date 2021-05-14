use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_ecs_tilemap::prelude::*;

#[derive(Default)]
pub struct MapData {
    pub road_path: Vec<IVec2>,
    // TODO: Add more map things like: Towns, Dungeons, etc.
}

pub struct MapSeed(pub u32);

#[derive(SystemParam)]
pub struct MapQuery<'a> {
    map_query_set: QuerySet<(
        Query<'a, (Entity, &'static mut Map)>,
        Query<'a, (Entity, &'static Map)>,
    )>,
}

pub const MAP_LAYER: u32 = 0;
pub const ROAD_LAYER: u32 = 1;

impl<'a> MapQuery<'a> {
    pub fn get_layer(&self, layer_id: u32) -> Option<(Entity, &Map)> {
        self.map_query_set.q1().iter().find(|map| map.1.settings.layer_id == layer_id)
    }

    pub fn get_layer_mut(&mut self, layer_id: u32) -> Option<(Entity, Mut<'_, Map>)> {
        self.map_query_set.q0_mut().iter_mut().find(|map| map.1.settings.layer_id == layer_id)
    }
}
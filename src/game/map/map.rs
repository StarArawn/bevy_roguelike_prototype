use bevy::prelude::*;
#[derive(Default)]
pub struct MapData {
    pub road_path: Vec<IVec2>,
    // TODO: Add more map things like: Towns, Dungeons, etc.
}

pub struct MapSeed(pub u32);

pub mod layers {
    pub const GROUND: u32 = 0;
    pub const ROAD: u32 = 1;
}

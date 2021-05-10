#[derive(Default)]
pub struct MapData {
    pub road_path: Vec<(i32, i32)>,
    // TODO: Add more map things like: Towns, Dungeons, etc.
}

pub struct MapLayer;
pub struct RoadLayer;

pub struct MapSeed(pub u32);
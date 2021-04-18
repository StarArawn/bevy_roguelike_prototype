use super::TilemapAtlasHandles;
use bevy::{asset::LoadState, prelude::*};

pub fn get_has_map_assets(
    asset_server: Res<AssetServer>,
    tilemap_atlas_handles: Res<TilemapAtlasHandles>,
) -> bool {
    asset_server.get_group_load_state(tilemap_atlas_handles.handles.iter().map(|handle| handle.id))
        == LoadState::Loaded
}

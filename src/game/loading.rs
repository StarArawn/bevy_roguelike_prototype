use bevy::{asset::LoadState, prelude::*};
use super::{GameState, LoadingHandles};

// A system to determin if we have finished loading and should change states.
pub fn loading(
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<State<GameState>>,
    loading_handles: Res<LoadingHandles>,
) {
    dbg!(loading_handles.0.len());
    if asset_server.get_group_load_state(loading_handles.0.iter().map(|handle_id| handle_id.id)) == LoadState::Loaded
    {
        game_state.set(GameState::SpawnMap).unwrap();
    }
}

use bevy::prelude::*;

mod player;
pub use player::{Player, PlayerBundle};

fn spawn_player(mut commands: Commands) {
    commands.spawn_bundle(PlayerBundle::default());
}
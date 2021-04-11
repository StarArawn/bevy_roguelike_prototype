use bevy::prelude::*;

#[derive(Default, Clone, Debug)]
pub struct Player {}

#[derive(Bundle, Default, Clone, Debug)]
pub struct PlayerBundle {
    player: Player,
    transform: Transform,
}
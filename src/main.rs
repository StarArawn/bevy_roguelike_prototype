#![allow(dead_code)]

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod game;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            width: 1270.0,
            height: 720.0,
            title: String::from("rogue-like-prototype"),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_plugin(game::GamePlugin)
        .run();
}

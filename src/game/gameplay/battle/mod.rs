use bevy::prelude::*;

use crate::game::GameState;

pub enum BattleLocation {
    Mountains,
}

pub fn get_battle_location_texture(battle_location: BattleLocation) -> &'static str {
    match battle_location {
        BattleLocation::Mountains => {
            "textures/backgrounds/battle1.png"
        },
        _ => panic!("No matching background texture found for battle location.")
    }
}

pub fn spawn_battle_screen(
    battle_location: BattleLocation,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle: Handle<Texture> = asset_server.load(get_battle_location_texture(battle_location));
    let background_sprite = materials.add(texture_handle.into());
    commands.spawn()
        .insert_bundle(SpriteBundle {
            material: background_sprite,
            transform: Transform::from_xyz(0.0, 0.0, 10.0),
            ..Default::default()
        })
        .insert(GameState::BattleView);
}
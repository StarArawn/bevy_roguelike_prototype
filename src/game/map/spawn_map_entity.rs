use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::GameState;

#[derive(Default, Clone)]
pub struct TilemapAtlasHandles {
    pub handles: Vec<HandleUntyped>,
}

pub fn spawn_map_entity(
    mut commands: Commands,
    mut game_state: ResMut<State<GameState>>,
    mut textures: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    let tilemap_atlas_handle = asset_server.get_handle("textures/map/road-sprites.png");
    let atlas_texture = textures.get_mut(tilemap_atlas_handle.clone()).unwrap();
    atlas_texture.sampler.min_filter = bevy::render::texture::FilterMode::Nearest;
    let material_handle = materials.add(ColorMaterial::texture(tilemap_atlas_handle));

    let map_settings = MapSettings::new(UVec2::new(10, 10), UVec2::new(32, 32), Vec2::new(16.0, 16.0), Vec2::new(96.0, 256.0), 0);

    let layer_count = 2;

    for i in 0..layer_count {
        let mut layer_settings = map_settings.clone();
        layer_settings.layer_id = i;
        let map_layer_entity = commands.spawn().id();
        let mut map_layer = Map::new(layer_settings);
        map_layer.build(&mut commands, &mut meshes, material_handle.clone(), map_layer_entity, false);
        commands.entity(map_layer_entity).insert_bundle(MapBundle {
            map: map_layer,
            transform: Transform::from_xyz(-2560.0, -2560.0, i as f32),
            ..Default::default()
        });
    }

    game_state.set(GameState::GenerateMap);
}

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::{GameState, gameplay::enemy::spawner};
use super::{MapData, generate_map, generate_road, map::layers};

#[derive(Default, Clone)]
pub struct TilemapAtlasHandles {
    pub handles: Vec<HandleUntyped>,
}

pub fn spawn_map_entity(
    mut commands: Commands,
    mut game_state: ResMut<State<GameState>>,
    mut textures: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut map_query: MapQuery,
) {
    let tilemap_atlas_handle = asset_server.get_handle("textures/map/road-sprites.png");
    let atlas_texture = textures.get_mut(tilemap_atlas_handle.clone()).unwrap();
    atlas_texture.sampler.min_filter = bevy::render::texture::FilterMode::Nearest;
    let material_handle = materials.add(ColorMaterial::texture(tilemap_atlas_handle));

    let layer_settings = LayerSettings::new(UVec2::new(10, 10), UVec2::new(32, 32), Vec2::new(16.0, 16.0), Vec2::new(96.0, 256.0));

    let ground_layer_entity = commands.spawn().id();
    let mut ground_layer_builder = LayerBuilder::<TileBundle>::new(
        &mut commands,
        ground_layer_entity,
        layer_settings.clone()
    );

    let road_layer_entity = commands.spawn().id();
    let mut road_settings = layer_settings.clone();
    road_settings.layer_id = layers::ROAD;
    let mut road_layer_builder = LayerBuilder::<TileBundle>::new(
        &mut commands,
        ground_layer_entity,
        road_settings,
    );

    let size = layer_settings.map_size * layer_settings.chunk_size * layer_settings.tile_size.as_u32();
    let map_center_transform = Transform::from_xyz(-(size.x as f32) / 2.0, -(size.y as f32) / 2.0, 0.0);

    generate_map(&mut ground_layer_builder);
    let road_path = generate_road(&ground_layer_builder, &mut road_layer_builder);

    for _ in 0..10 {
        let random_road_index = fastrand::usize(0..road_path.len() - 1);
        let road_point = road_path[random_road_index];
        spawner::spawn(
            &mut commands,
            &asset_server,
            &mut materials,
            Vec2::new(road_point.x as f32 * 16.0, road_point.y as f32 * 16.0) + Vec2::new(8.0, 8.0),
        );
    }

    map_query.create_layer(&mut commands, ground_layer_builder, material_handle.clone());
    map_query.create_layer(&mut commands, road_layer_builder, material_handle.clone());

    commands.entity(ground_layer_entity).insert(Transform {
        translation: Vec3::new(map_center_transform.translation.x, map_center_transform.translation.y, layers::GROUND as f32),
        ..map_center_transform
    });
    commands.entity(ground_layer_entity).insert(MapData {
        road_path,
    });

    commands.entity(road_layer_entity).insert(Transform {
        translation: Vec3::new(map_center_transform.translation.x, map_center_transform.translation.y, layers::ROAD as f32),
        ..map_center_transform
    });

    game_state.set(GameState::MapView).unwrap();
}

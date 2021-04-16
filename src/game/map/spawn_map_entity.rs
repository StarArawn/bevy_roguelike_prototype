use bevy::{prelude::*};
use bevy_tilemap::prelude::*;

use crate::game::{GameState, camera::CustomOrthographicCameraBundle};

#[derive(Default, Clone)]
pub struct TilemapAtlasHandles {
    pub handles: Vec<HandleUntyped>,
}

pub fn spawn_map_entity(
    mut commands: Commands,
    tilemap_atlas_handles: Res<TilemapAtlasHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Texture>>,
) {
    let tilemap_atlas_handle = tilemap_atlas_handles.handles[0].clone().typed::<Texture>();
    let atlas_texture = textures.get_mut(tilemap_atlas_handle.clone()).unwrap();
    atlas_texture.sampler.min_filter = bevy::render::texture::FilterMode::Nearest;
    atlas_texture.sampler.mag_filter = bevy::render::texture::FilterMode::Nearest;
    atlas_texture.sampler.mipmap_filter = bevy::render::texture::FilterMode::Nearest;

    let texture_atlas = TextureAtlas::from_grid(tilemap_atlas_handle, Vec2::new(16.0, 16.0), 6, 16);

    let atlas_handle = texture_atlases.add(texture_atlas);

    let tilemap = Tilemap::builder()
        .auto_chunk()
        .topology(GridTopology::Square)
        .dimensions(10, 10)
        .chunk_dimensions(32, 32, 1)
        .texture_dimensions(16, 16)
        .z_layers(2)
        .add_layer(
            TilemapLayer {
                kind: LayerKind::Dense,
                ..Default::default()
            },
            0,
        )
        .add_layer(
            TilemapLayer {
                kind: LayerKind::Sparse,
                ..Default::default()
            },
            1,
        )
        .texture_atlas(atlas_handle)
        .finish()
        .unwrap();
        
    let tilemap_components = TilemapBundle {
        tilemap,
        visible: Visible {
            is_visible: true,
            is_transparent: true,
        },
        transform: Default::default(),
        global_transform: Default::default(),
    };

    commands
        .spawn()
        .insert_bundle(CustomOrthographicCameraBundle::new_2d());
    commands
        .spawn()
        .insert(GameState::MapView)
        .insert_bundle(tilemap_components);
}

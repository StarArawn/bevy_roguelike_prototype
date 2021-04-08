use bevy::{asset::LoadState, prelude::*};
use bevy_tilemap::prelude::*;

#[derive(Default, Clone)]
pub struct TilemapAtlasHandles {
    pub handles: Vec<HandleUntyped>,
    loaded: bool,
}

pub fn spawn_map_entity(
    mut commands: Commands,
    mut tilemap_atlas_handles: ResMut<TilemapAtlasHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>
) {
    if tilemap_atlas_handles.loaded {
        return;
    }

    if let LoadState::Loaded =
        asset_server.get_group_load_state(tilemap_atlas_handles.handles.iter().map(|handle| handle.id))
    {
        let tilemap_atlas_handle = tilemap_atlas_handles.handles[0].clone().typed::<Texture>();
        let texture_atlas = TextureAtlas::from_grid(tilemap_atlas_handle, Vec2::new(16.0, 16.0), 6, 16);
    
        let atlas_handle = texture_atlases.add(texture_atlas);

        let tilemap = Tilemap::builder()
            .auto_chunk()
            .topology(GridTopology::Square)
            .dimensions(2, 2)
            .chunk_dimensions(8, 8, 1)
            .texture_dimensions(16, 16)
            .z_layers(1)
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
            .insert_bundle(OrthographicCameraBundle::new_2d());
        commands
            .spawn()
            .insert_bundle(tilemap_components);

        tilemap_atlas_handles.loaded = true;
    }
}

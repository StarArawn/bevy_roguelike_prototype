use bevy::{prelude::*, render};

use crate::game::{GameState};

pub mod spawner;

#[derive(Default)]
pub struct Enemy {}

// pub struct EnemyRenderer {
//     parent: Entity,
//     entity: Option<Entity>,
// }

// impl EnemyRenderer {
//     pub fn new(parent: Entity) -> Self {
//         Self {
//             parent,
//             entity: None,
//         }
//     }
// } 

// impl EntityRenderingState for EnemyRenderer {
//     fn spawn(
//         &mut self,
//         game_state: &GameState,
//         commands: &mut Commands,
//         asset_server: &Res<AssetServer>,
//         materials: &mut ResMut<Assets<ColorMaterial>>,
//         texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
//     ) {
//         commands.entity(self.parent).with_children(|child_builder| {
//             match game_state {
//                 GameState::MapView => {
//                     let texture_handle: Handle<Texture> = asset_server.load("textures/spider_sprite.png");
//                     let enemy_sprite_material = materials.add(texture_handle.into());
//                     let render_entity = child_builder    
//                         .spawn_bundle(SpriteBundle {
//                             material: enemy_sprite_material,
//                             ..Default::default()
//                         }).id();
//                     self.entity = Some(render_entity);
//                 },
//                 _ => {},
//             }
//         });
//     }

//     fn entity(&self) -> Entity where Self: 'static {
//         self.entity.unwrap()
//     }
// }

pub fn spawn_enemy(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    position: Vec2,
) {
    let enemy_entity = commands
        .spawn()
        .insert(GlobalTransform::default())
        .insert(Transform::from_xyz(position.x, position.y, 12.0))
        .insert(Enemy::default())
        .with_children(|child_builder| {
            let texture_handle: Handle<Texture> = asset_server.load("textures/spider_sprite.png");
            let enemy_sprite_material = materials.add(texture_handle.into());
            let render_entity = child_builder    
                .spawn_bundle(SpriteBundle {
                    material: enemy_sprite_material,
                    ..Default::default()
                }).id();
        })
        .id();

    // let mut enemy_render_entity = EnemyRenderer::new(enemy_entity);
    // enemy_render_entity.spawn(&GameState::MapView, commands, asset_server, materials, texture_atlases);

    // commands.entity(enemy_entity)
    //     .insert(Box::new(enemy_render_entity));
}
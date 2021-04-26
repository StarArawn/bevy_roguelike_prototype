use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::game::GameState;
use super::spawn_enemy;

#[derive(Default)]
pub struct Spawner {
    wait_time: f64, // How long until a enemy spawns
    last_time: f64, // The time since the last spawn
    limit: u32,     // The maximum enemy's that can spawn from this spawner.
    current: u32,   // The current count of spawned enemies.
    _range: f32,    // How far away from the spawner an enemy can spawn.
}

pub fn spawn(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec2,
) {
    let texture_handle: Handle<Texture> = asset_server.load("textures/cave_sprite.png");
    let cave_sprite_material = materials.add(texture_handle.into());
    commands
        .spawn_bundle(SpriteBundle {
            material: cave_sprite_material,
            transform: Transform::from_xyz(position.x, position.y, 10.0),
            ..Default::default()
        })
        .insert(GameState::MapView)
        .insert(Spawner {
            wait_time: 10.0,
            last_time: 0.0,
            limit: 3,
            current: 0,
            _range: 2.0,
        });
}

pub fn tick(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut spawner_query: Query<(&Transform, &mut Spawner)>,
    time: Res<Time>,
) {
    let current_time = time.seconds_since_startup();

    let spawn_pos = vec![
        Vec2::new(16.0, 0.0),
        Vec2::new(-16.0, 0.0),
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, 16.0),
        Vec2::new(0.0, -16.0),
    ];

    let mut random = thread_rng();

    for (transform, mut spawner) in spawner_query.iter_mut() {
        let spawner_elapsed_time = current_time - spawner.last_time;
        if spawner_elapsed_time > spawner.wait_time && spawner.current < spawner.limit {
            let offset = spawn_pos[random.gen_range(0..4)];
            spawn_enemy(
                &mut commands,
                &asset_server,
                &mut texture_atlases,
                &mut materials,
                Vec2::new(transform.translation.x, transform.translation.y) + offset,
            );
            spawner.last_time = current_time;
            spawner.current += 1;
        }
    }
}

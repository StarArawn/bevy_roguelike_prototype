use std::thread::current;

use bevy::{prelude::*, sprite};
use bevy_tilemap::prelude::*;
use noise::{Fbm, MultiFractal, NoiseFn, Seedable};
use rand::{Rng, prelude::ThreadRng, thread_rng};
use crate::game::GameState;

fn get_island_shape(x: f64, y: f64) -> f64 {
    let a = 1.0;
    let b = 1.2;
    let value = x.abs().max(y.abs());

    value.powf(a) / value.powf(a) + (b - b * value).powf(a)
}

fn generate_road(tile_map: &mut Tilemap, random: &mut ThreadRng) {
    let angle_increment: u32 = 15;
    let random_angle = random.gen_range(0..360) as f32;

    let mut angles = Vec::new();
    for angle_index in 0..(360 / angle_increment) {
        let mut current_angle = (angle_increment * angle_index) as f32 + random_angle;
        current_angle = current_angle % 360.0;
        if current_angle < 0.0 {
            current_angle += 360.0;
        }

        angles.push(current_angle);
    }

    angles.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut road_points = Vec::new();
    for angle in angles {
        let current_direction = Vec2::new(angle.to_radians().cos(), angle.to_radians().sin()).normalize();

        for ray_index in 2..1000 {
            let check_position: Vec2 = current_direction * (ray_index as f32);

            let tile = tile_map.get_tile((check_position.x as i32, check_position.y as i32), 0);

            match tile {
                Some(tile) => {
                    if tile.index == 19 {
                        let range = 30..(ray_index - 2);
                        let random_ray_position = random.gen_range(range);
                        let road_position: Vec2 = current_direction * (random_ray_position as f32);
                        road_points.push(road_position);
                        break;
                    }
                },
                None => {
                    break;
                }
            }
        }
    }
    
    for road_point in road_points {
        let tile = Tile {
            point: (road_point.x as i32, road_point.y as i32),
            sprite_index: 36,
            sprite_order: 1,
            ..Default::default()
        };

        tile_map.insert_tile(tile).unwrap();
    }
}

pub fn generate_map(mut game_state: ResMut<State<GameState>>, mut map_query: Query<&mut Tilemap>) {
    if *game_state.current() == GameState::Playing {
        return;
    }

    for mut map in map_query.iter_mut() {

        // Generate a seed for the map
        let mut random = thread_rng();
        let seed: u32 = random.gen();

        // Create fbm noise 
        let mut fbm = Fbm::new();
        fbm = fbm.set_seed(seed);
        fbm = fbm.set_frequency(0.2);

        let chunk_width = map.chunk_width() as i32;
        let chunk_height = map.chunk_height() as i32;
        let map_width = map.width().unwrap() as i32;
        let map_height = map.height().unwrap() as i32;

        let actual_width = map_width * chunk_width;
        let actual_height = map_height * chunk_height;

        let half_actual_width = actual_width / 2;
        let half_actual_height = actual_height / 2;

        let half_map_width = map_width / 2;
        let half_map_height = map_height / 2;

        let mut tiles = Vec::new();
        for x in -half_actual_width..half_actual_width {
            for y in -half_actual_height..half_actual_height {
                let high_x = x as f64;
                let high_y = y as f64;
                let mask = get_island_shape(high_x / 60.0, high_y / 60.0);
                let noise_value = fbm.get([high_x / 15.0, high_y / 15.0]) - (1.0 - mask);

                // Create Tile
                let mut  tile = Tile {
                    point: (x, y),
                    sprite_index: 19,
                    ..Default::default()
                };

                if noise_value > 0.0 {

                    if noise_value > 0.9 {
                        tile.sprite_index = 23;    
                    } else if noise_value > 0.7 {
                        tile.sprite_index = 22;
                    } else if noise_value > 0.6 {
                        tile.sprite_index = 21;
                    } else if noise_value > 0.4 {
                        tile.sprite_index = 20;
                    } else {
                        tile.sprite_index = 18;
                    }
                }

                tiles.push(tile);
            }
        }
        map.insert_tiles(tiles).unwrap();

        generate_road(&mut map, &mut random);

        for x in -half_map_width..half_map_width {
            for y in -half_map_height..half_map_height {
                map.spawn_chunk((x, y)).unwrap();
            }
        }

        game_state.set(GameState::Playing).unwrap();
    }

}
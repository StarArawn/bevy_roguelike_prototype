use bevy_tilemap::prelude::*;
use bevy::{prelude::*};
use noise::{Fbm, MultiFractal, NoiseFn, Seedable};
use pathfinding::prelude::{absdiff, astar};
use rand::{Rng, prelude::ThreadRng, thread_rng};
use crate::game::GameState;
use super::Map;

fn get_island_shape(x: f64, y: f64) -> f64 {
    let a = 1.0;
    let b = 1.2;
    let value = x.abs().max(y.abs());

    value.powf(a) / value.powf(a) + (b - b * value).powf(a)
}

fn generate_road(tilemap: &mut Tilemap, random: &mut ThreadRng) -> Vec<Vec2> {
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

            let tile = tilemap.get_tile((check_position.x as i32, check_position.y as i32), 0);

            match tile {
                Some(tile) => {
                    if tile.index == 19 {
                        let range = 30..(ray_index - 2);
                        dbg!(&range);
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
    
    for road_point in road_points.iter() {
        let tile = Tile {
            point: (road_point.x as i32, road_point.y as i32),
            sprite_index: 36,
            sprite_order: 1,
            ..Default::default()
        };

        tilemap.insert_tile(tile).unwrap();
    }

    road_points
}

pub fn find_neighbors(pos: (i32, i32), tilemap: &mut Tilemap) -> Vec<((i32, i32), i32)> {
    // North
    let neighbor_north = tilemap.get_tile((pos.0, pos.1 + 1), 0).unwrap().clone();
    // South
    let neighbor_south = tilemap.get_tile((pos.0, pos.1 - 1), 0).unwrap().clone();
    // West
    let neighbor_west = tilemap.get_tile((pos.0 - 1, pos.1), 0).unwrap().clone();
    // East
    let neighbor_east = tilemap.get_tile((pos.0 + 1, pos.1), 0).unwrap().clone();

    // North East
    let neighbor_north_east = tilemap.get_tile((pos.0 + 1, pos.1 + 1), 0).unwrap().clone();
    // North West
    let neighbor_north_west = tilemap.get_tile((pos.0 - 1, pos.1 + 1), 0).unwrap().clone();
    // South East
    let neighbor_south_east = tilemap.get_tile((pos.0 + 1, pos.1 - 1), 0).unwrap().clone();
    // South West
    let neighbor_south_west = tilemap.get_tile((pos.0 - 1, pos.1 - 1), 0).unwrap().clone();
    
    let mut neighbors = Vec::new();

    if neighbor_north.index != 19 {
        neighbors.push((pos.0, pos.1 + 1));
    }
    if neighbor_south.index != 19 {
        neighbors.push((pos.0, pos.1 - 1));
    }
    if neighbor_west.index != 19 {
        neighbors.push((pos.0 - 1, pos.1));
    }
    if neighbor_east.index != 19 {
        neighbors.push((pos.0 + 1, pos.1));
    }

    if neighbor_north_east.index != 19 {
        neighbors.push((pos.0 + 1, pos.1 + 1));
    }
    if neighbor_north_west.index != 19 {
        neighbors.push((pos.0 - 1, pos.1 + 1));
    }
    if neighbor_south_east.index != 19 {
        neighbors.push((pos.0 + 1, pos.1 - 1));
    }
    if neighbor_south_west.index != 19 {
        neighbors.push((pos.0 - 1, pos.1 - 1));
    }

    neighbors.into_iter().map(|p| (p, 1)).collect()
}

pub fn find_road_path(road_points: &Vec<Vec2>, tilemap: &mut Tilemap) -> Vec<(i32, i32)> {
    let mut road_path = Vec::new();

    let mut starting_point = (road_points[0].x as i32, road_points[0].y as i32);
    for road_point_index in 1..road_points.len() {
        let goal = (road_points[road_point_index].x as i32, road_points[road_point_index].y as i32);
        // Do pathfinding
        let path = astar(
            &starting_point,
            |&(x, y)| {
                find_neighbors((x, y), tilemap)
            },
            |&(x, y)| absdiff(x, goal.0) + absdiff(y, goal.1),
            |&p| p == goal
        );

        road_path.extend(path.unwrap().0);

        starting_point = goal.clone();
    }

    for road_point in road_path.iter() {

        let has_no_tile = tilemap.get_tile(*road_point, 1).is_none();

        if has_no_tile {
            let tile = Tile {
                point: road_point.clone(),
                sprite_index: 7,
                sprite_order: 1,
                ..Default::default()
            };

            tilemap.insert_tile(tile).unwrap();
        }
    }

    road_path
}

pub fn generate_map(
    mut game_state: ResMut<State<GameState>>,
    mut map: ResMut<Map>,
    mut tilemap_query: Query<&mut Tilemap>
) {
    for mut tilemap in tilemap_query.iter_mut() {

        // Generate a seed for the map
        let mut random = thread_rng();
        let seed: u32 = random.gen();

        // Create fbm noise 
        let mut fbm = Fbm::new();
        fbm = fbm.set_seed(seed);
        fbm = fbm.set_frequency(0.2);

        let chunk_width = tilemap.chunk_width() as i32;
        let chunk_height = tilemap.chunk_height() as i32;
        let map_width = tilemap.width().unwrap() as i32;
        let map_height = tilemap.height().unwrap() as i32;

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
                    sprite_index: 19, // Water
                    ..Default::default()
                };

                if noise_value > 0.0 {
                    if noise_value > 0.9 {
                        tile.sprite_index = 23; // Snow   
                    } else if noise_value > 0.7 {
                        tile.sprite_index = 22; // Rock 2
                    } else if noise_value > 0.6 {
                        tile.sprite_index = 21; // Rock 1
                    } else if noise_value > 0.4 {
                        tile.sprite_index = 20; // Forest
                    } else {
                        tile.sprite_index = 18; // Grass
                    }
                }

                tiles.push(tile);
            }
        }
        tilemap.insert_tiles(tiles).unwrap();

        let mut road_points = generate_road(&mut tilemap, &mut random);
        road_points.push(road_points[0]);

        let road_path = find_road_path(&road_points, &mut tilemap);
        map.road_path = road_path;

        for x in -half_map_width..half_map_width {
            for y in -half_map_height..half_map_height {
                tilemap.spawn_chunk((x, y)).unwrap();
            }
        }

        game_state.set(GameState::Playing).unwrap();
    }

}
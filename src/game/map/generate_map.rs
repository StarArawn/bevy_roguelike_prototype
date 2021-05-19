use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use noise::{Fbm, MultiFractal, NoiseFn, Seedable};
use pathfinding::prelude::{absdiff, astar};

fn get_island_shape(x: f64, y: f64) -> f64 {
    let a = 1.0;
    let b = 1.2;
    let value = x.abs().max(y.abs());

    value.powf(a) / value.powf(a) + (b - b * value).powf(a)
}

pub fn generate_road(
    ground_layer: &LayerBuilder<TileBundle>,
    road_layer: &mut LayerBuilder<TileBundle>,
) -> Vec<IVec2> {
    let mut road_points = Vec::new();          
    let angle_increment: u32 = 15;
    let random_angle = fastrand::u32(0..360) as f32;

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

    let map_center = (ground_layer.settings.map_size * ground_layer.settings.chunk_size).as_f32() / 2.0;
    
    for angle in angles {
        let current_direction =
            Vec2::new(angle.to_radians().cos(), angle.to_radians().sin()).normalize();

        for ray_index in 2..1000 {
            let check_position: Vec2 = map_center + (current_direction * ray_index as f32);

            if let Ok(tile_bundle) = ground_layer.get_tile(check_position.as_u32()) {
                if tile_bundle.tile.texture_index == 19 {
                    let range = 30..(ray_index - 2);
                    let random_ray_position = fastrand::i32(range);
                    let road_position: Vec2 = map_center + current_direction * (random_ray_position as f32);
                    road_points.push(road_position);
                    break;
                }
            } else {
                break;
            }
        }
    }

    for road_point in road_points.iter() {
        let tile = Tile {
            texture_index: 36,
            ..Default::default()
        };
        road_layer.set_tile(road_point.as_u32(), tile.into(), true).unwrap();
    }

    road_points.push(road_points[0]);

    let road_path = find_road_path(ground_layer, road_layer, &road_points);

    road_path
}

pub fn find_road_path(
    ground_layer: &LayerBuilder<TileBundle>,
    road_layer: &mut LayerBuilder<TileBundle>,
    road_points: &Vec<Vec2>,
) -> Vec<IVec2> {
    let mut road_path = Vec::new();
    let mut starting_point = (road_points[0].x as i32, road_points[0].y as i32);

    for road_point_index in 1..road_points.len() {
        let goal = (
            road_points[road_point_index].x as i32,
            road_points[road_point_index].y as i32,
        );
        // Do pathfinding
        let path = astar(
            &starting_point,
            |&(x, y)| {
                let neighbors = ground_layer.get_tile_neighbors(UVec2::new(x as u32, y as u32));
                
                neighbors
                    .iter()
                    .filter(|(_, tile_data)| {
                        if let Some((_, tile_bundle)) = tile_data {
                            // 19 is water.
                            if tile_bundle.tile.texture_index != 19 {
                                return true;
                            }
                        }
                        return false;
                    })
                    .map(|(p, _)| ((p.x, p.y), 1))
                    .collect::<Vec<((i32, i32), i32)>>().into_iter()
            },
            |&(x, y)| absdiff(x, goal.0) + absdiff(y, goal.1),
            |&p| p == goal,
        ).unwrap().0.iter().map(|(x, y)| IVec2::new(*x, *y)).collect::<Vec<IVec2>>();

        road_path.extend(path);

        starting_point = goal.clone();
    }
    
    for road_point in road_path.iter() {
        let has_no_tile = road_layer.get_tile(road_point.as_u32()).is_err();

        if has_no_tile {
            let tile = Tile {
                texture_index: 7,
                ..Default::default()
            };

            road_layer.set_tile(road_point.as_u32(), tile.into(), true).unwrap();
        }
    }

    let half_map_size = (ground_layer.settings.map_size * ground_layer.settings.chunk_size).as_i32() / 2;
    road_path = road_path.iter().map(|vec| IVec2::new(vec.x - half_map_size.x, vec.y - half_map_size.y)).collect();
    
    road_path
}

pub fn generate_map(
    ground_layer: &mut LayerBuilder<TileBundle>,
) {
    // Generate a seed for the map
    let seed: u32 = fastrand::u32(..);
    fastrand::seed(seed as u64);
    // Create fbm noise
    let mut fbm = Fbm::new();
    fbm = fbm.set_seed(seed);
    fbm = fbm.set_frequency(0.2);

    let chunk_width = ground_layer.settings.chunk_size.x;
    let chunk_height = ground_layer.settings.chunk_size.y;
    let map_width = ground_layer.settings.map_size.x;
    let map_height = ground_layer.settings.map_size.x;

    let actual_width = map_width * chunk_width;
    let actual_height = map_height * chunk_height;

    let half_actual_width = actual_width / 2;
    let half_actual_height = actual_height / 2;

    for x in 0..actual_width {
        for y in 0..actual_height {
            let high_x = x as f64 - half_actual_width as f64;
            let high_y = y as f64 - half_actual_height as f64;
            let mask = get_island_shape(high_x / 60.0, high_y / 60.0);
            let noise_value = fbm.get([high_x / 15.0, high_y / 15.0]) - (1.0 - mask);

            // Create Tile
            let mut tile = Tile {
                texture_index: 19, // Water
                ..Default::default()
            };

            if noise_value > 0.0 {
                if noise_value > 0.9 {
                    tile.texture_index = 23; // Snow
                } else if noise_value > 0.7 {
                    tile.texture_index = 22; // Rock 2
                } else if noise_value > 0.6 {
                    tile.texture_index = 21; // Rock 1
                } else if noise_value > 0.4 {
                    tile.texture_index = 20; // Forest
                } else {
                    tile.texture_index = 18; // Grass
                }
            }

            let _ = ground_layer.set_tile(UVec2::new(x, y), tile.into(), true);
        }
    }
}

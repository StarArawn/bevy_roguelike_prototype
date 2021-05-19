use bevy::prelude::*;

use crate::game::map::{MapData};

use super::PlayerSprite;

pub fn movement(
    time: Res<Time>,
    mut player_query: Query<(&mut PlayerSprite, &mut Transform)>,
    map_data_query: Query<&MapData>,
) {
    if let Ok(map_data) = map_data_query.single() {
        if map_data.road_path.len() > 0 {
            for (mut player, mut transform) in player_query.iter_mut() {
                let current_road_position = map_data.road_path[player.current_position];
                let current_road_position = Vec2::new(
                    (current_road_position.x as f32 * 16.0) + 8.0,
                    (current_road_position.y as f32 * 16.0) + 8.0,
                );
                let mut player_position = transform.translation.truncate();


                let direction = (current_road_position - player_position).normalize();
                player_position += direction * 100.0 * time.delta_seconds();

                transform.translation = player_position.extend(10.0);            

                let distance = current_road_position.distance_squared(player_position);
                if distance <= 1.0 {
                    player.current_position += 1;
                    if player.current_position >= map_data.road_path.len() {
                        player.current_position = 0;
                    }
                }
            }
        }
    }
}

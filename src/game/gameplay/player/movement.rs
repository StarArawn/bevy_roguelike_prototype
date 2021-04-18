use crate::game::{map::Map, timing::Timing};
use bevy::prelude::*;

use super::Player;

pub fn movement(
    mut timing: ResMut<Timing>,
    map: Res<Map>,
    mut player_query: Query<(&mut Player, &mut Transform)>,
) {
    if !timing.should_update {
        return;
    }

    timing.should_update = false;

    if map.road_path.len() > 0 {
        for (mut player, mut transform) in player_query.iter_mut() {
            player.current_position += 1;
            if player.current_position >= map.road_path.len() {
                player.current_position = 0;
            }

            let current_road_position = map.road_path[player.current_position];
            transform.translation = Vec3::new(
                (current_road_position.0 as f32 * 16.0) + 8.0,
                (current_road_position.1 as f32 * 16.0) + 8.0,
                10.0,
            );
        }
    }
}

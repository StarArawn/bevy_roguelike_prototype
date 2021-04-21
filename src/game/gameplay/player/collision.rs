use bevy::prelude::*;
use crate::game::{GameState, gameplay::{enemy::Enemy, scenes}};
use super::PlayerSprite;

pub fn check(
    mut battle_event_writer: EventWriter<scenes::battle::BattleEvent>,
    mut game_state: ResMut<State<GameState>>,
    player_query: Query<(&PlayerSprite, &Transform)>,
    enemy_query: Query<(Entity, &Enemy, &Transform)>,
) {
    let mut enemies_within_range = Vec::new();
    for (_, player_transform) in player_query.iter() {
        for (enemy_entity, _, enemy_transform) in enemy_query.iter() {
            let distance = player_transform.translation.distance_squared(enemy_transform.translation);
            if distance <= 32.0 {
                enemies_within_range.push((distance, enemy_entity, enemy_transform));
            }
        }

        enemies_within_range.sort_by(|a, b| { a.0.partial_cmp(&b.0).unwrap() });

        for (distance, enemy_entity, _enemy_transform) in enemies_within_range.iter() {
            if *distance < 8.0 {
                game_state.set(GameState::BattleView).unwrap();
                battle_event_writer.send(scenes::battle::BattleEvent {
                    battle_location: scenes::battle::BattleLocation::Mountains,
                    enemy_entity: *enemy_entity,
                });
            }
        }
    }
}
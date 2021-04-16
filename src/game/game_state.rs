use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Loading,
    Generating,
    MapView,
    BattleView,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Loading
    }
}

pub fn update_visibility_for_state(
    game_state: Res<State<GameState>>,
    mut query: Query<(&mut Visible, &mut Transform, &GameState)>,
) {
    for (mut visible, mut transform, target_game_state) in query.iter_mut() {
        if game_state.current() == target_game_state {
            visible.is_visible = true;
        } else {
            visible.is_visible = false;
        }
    }
}
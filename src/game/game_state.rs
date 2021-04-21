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

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Loading,
    GenerateMap,
    MapView,
    BattleView,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Loading
    }
}

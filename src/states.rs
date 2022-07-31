#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    MainMenu,
    MainGame,
    ControlMenu,
    GameOver,
}

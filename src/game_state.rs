#[derive(Debug, PartialEq, Copy, Clone)]
#[allow(dead_code)]
pub enum GameState {
    Menu,
    Playing,
    GameOver,
    Quit,
}

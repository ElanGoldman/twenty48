use crate::game::Game;

#[derive(Debug, Clone)]
pub struct GameSnapshot {
    pub board: Vec<Vec<u32>>,
    pub score: u32,
}

impl GameSnapshot {
    pub fn from_game(game: &Game) -> Self {
        Self {
            board: game.board.clone(),
            score: game.score,
        }
    }
}
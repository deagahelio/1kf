use ggez::{self, graphics, event, GameResult, Context};
use crate::board::Board;

pub struct GameState {
    board: Board,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            board: Board::new([0.0, 0.0], 8.0),
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);
        self.board.draw(ctx)?;
        graphics::present(ctx)
    }
}
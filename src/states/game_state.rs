use ggez::{self, graphics, event, GameResult, Context};
use crate::board::Board;
use crate::piece::Shape;
use crate::generator::PieceGenerator;

pub struct GameState {
    board: Board,
    current: Shape,
    generator: Box<dyn PieceGenerator>,
}

impl GameState {
    pub fn new(generator: Box<dyn PieceGenerator>) -> Self {
        Self {
            board: Board::new([16.0, 16.0], 16.0),
            current: Shape::T,
            generator: generator,
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, (23, 23, 19).into());
        self.board.draw(ctx)?;
        graphics::present(ctx)
    }
}
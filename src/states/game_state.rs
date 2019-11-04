use ggez::{self, graphics, event, GameResult, Context};
use crate::board::Board;
use crate::piece::Shape;
use crate::generator::PieceGenerator;

pub struct GameState<G: PieceGenerator> {
    board: Board,
    current: Shape,
    generator: G,
}

impl<G: PieceGenerator> GameState<G> {
    pub fn new(mut generator: G) -> Self {
        Self {
            board: Board::new([16.0, 16.0], 16.0),
            current: generator.get_next(),
            generator: generator,
        }
    }
}

impl<G: PieceGenerator> event::EventHandler for GameState<G> {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, (23, 23, 19).into());
        self.board.draw(ctx)?;
        graphics::present(ctx)
    }
}
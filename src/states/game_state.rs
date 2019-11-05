use ggez::{self, graphics, event, GameResult, Context};
use ggez::input::mouse::MouseButton;
use crate::board::{Board};
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
            board: Board::new([32.0, 32.0], 32.0),
            current: generator.get_next(),
            generator: generator,
        }
    }
}

impl<G: PieceGenerator> event::EventHandler for GameState<G> {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if button == MouseButton::Left {
            if let Some((grid_x, grid_y)) = self.board.click(x, y) {
                if self.board.put(grid_x, grid_y, self.current).is_ok() {
                    self.current = self.generator.get_next();
                }
            }
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, (23, 23, 19).into());
        self.board.draw(ctx)?;
        graphics::present(ctx)
    }
}
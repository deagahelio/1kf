use ggez::{self, graphics, event, GameResult, Context};
use ggez::input::mouse::MouseButton;
use crate::board::Board;
use crate::piece::Shape;
use crate::generator::PieceGenerator;
use crate::queue::Queue;

pub struct GameState<G: PieceGenerator> {
    board: Board,
    queue: Queue,
    current: Shape,
    generator: G,
}

impl<G: PieceGenerator> GameState<G> {
    pub fn new(mut generator: G) -> Self {
        Self {
            board: Board::new([32.0, 32.0].into(), 32.0, 10, 20),
            queue: Queue::new([384.0, 32.0].into(), 32.0),
            current: generator.get_next(),
            generator,
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
                if let Some(rotation) = self.board.get_rotation(grid_y) {
                    if self.board.put(grid_x, rotation, self.current).is_ok() {
                        self.current = self.generator.get_next();
                        self.board.clear_lines();
                    }
                }
            }
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, (23, 23, 19).into());
        self.board.draw(ctx, &self.current)?;
        self.queue.draw(ctx, &self.generator)?;
        graphics::present(ctx)
    }
}
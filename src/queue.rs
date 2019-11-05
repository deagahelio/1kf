use ggez::{mint, graphics, Context, GameResult};
use ggez::graphics::{DrawMode, Mesh, Rect};
use crate::piece::{PIECE_SHAPES, PIECE_COLORS, Shape};

pub struct Queue {
    pos: mint::Point2<f32>,
    cell_size: f32,
}

impl Queue {
    pub fn new(pos: mint::Point2<f32>, cell_size: f32) -> Self {
        Self {
            pos,
            cell_size,
        }
    }

    pub fn draw(&self, ctx: &mut Context, current: &Shape) -> GameResult {
        let block_mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(self.pos.x, self.pos.y, self.cell_size, self.cell_size),
            graphics::WHITE
        )?;

        let shape = &PIECE_SHAPES[current];
        let color = PIECE_COLORS[current];

        for y in 0..shape.len() {
            let row = &shape[y];
            for x in 0..row.len() {
                if row[x] == 1 {
                    let pos = [x as f32 * self.cell_size, y as f32 * self.cell_size];
                    graphics::draw(ctx, &block_mesh, (pos, color))?;
                }
            }
        }

        Ok(())
    }
}
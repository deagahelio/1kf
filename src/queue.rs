use ggez::{mint, graphics, Context, GameResult};
use ggez::graphics::{DrawMode, Mesh, Rect};
use crate::generator::PieceGenerator;
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

    pub fn draw_piece(&self, ctx: &mut Context, piece: &Shape, x: f32, y: f32) -> GameResult<f32> {
        let block_mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(x, y, self.cell_size, self.cell_size),
            graphics::WHITE
        )?;

        let shape = &PIECE_SHAPES[piece];
        let color = PIECE_COLORS[piece];

        for (shape_y, row) in shape.iter().enumerate() {
            for (shape_x, col) in row.iter().enumerate() {
                if *col == 1 {
                    let pos = [shape_x as f32 * self.cell_size, shape_y as f32 * self.cell_size];
                    graphics::draw(ctx, &block_mesh, (pos, color))?;
                }
            }
        }

        Ok(shape.len() as f32 * self.cell_size)
    }

    pub fn draw(&self, ctx: &mut Context, generator: &impl PieceGenerator) -> GameResult {
        let mut offset = 0.0;
        
        for piece in generator.peek_next(5).iter() {
            offset += self.draw_piece(ctx, piece, self.pos.x, self.pos.y + offset)? + self.cell_size;
        }

        Ok(())
    }
}
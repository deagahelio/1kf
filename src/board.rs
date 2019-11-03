use ggez::{self, mint, graphics, Context, GameResult};
use ggez::graphics::{DrawMode, Mesh, Rect};
use anyhow::{Result, anyhow};
use crate::piece::Shape;

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;

pub struct Board {
    pos: mint::Point2<f32>,
    cell_size: f32,
    grid: [Option<Shape>; BOARD_WIDTH * BOARD_HEIGHT],
}

impl Board {
    pub fn new(pos: impl Into<mint::Point2<f32>>, cell_size: f32) -> Self {
        Self {
            pos: pos.into(),
            cell_size: cell_size,
            grid: [Some(Shape::T); BOARD_WIDTH * BOARD_HEIGHT],
        }
    }

    pub fn index(x: usize, y: usize) -> Result<usize> {
        let index = y * BOARD_WIDTH + x;
        if index >= BOARD_WIDTH * BOARD_HEIGHT {
            Err(anyhow!("out of bounds index"))
        } else {
            Ok(index)
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Result<Option<Shape>> {
        Ok(self.grid[Board::index(x, y)?])
    }

    pub fn set(&mut self, x: usize, y: usize, new: Shape) -> Result<Option<Shape>> {
        let index = Board::index(x, y)?;
        let old = self.grid[index];
        self.grid[index] = Some(new);
        Ok(old)
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let rect = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(self.pos.x, self.pos.y, self.cell_size, self.cell_size),
            graphics::WHITE
        )?;

        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                let block = self.get(x, y).unwrap();
                if block.is_some() {
                    graphics::draw(ctx, &rect, ([x as f32 * self.cell_size, y as f32 * self.cell_size],))?;
                }
            }
        }

        Ok(())
    }
}
use ggez::{mint, graphics, Context, GameResult};
use ggez::graphics::{DrawMode, Mesh, Rect};
use anyhow::{Result, anyhow};
use crate::piece::{PIECE_COLORS, Shape};

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
            grid: [None; BOARD_WIDTH * BOARD_HEIGHT],
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
        let block_mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(self.pos.x, self.pos.y, self.cell_size, self.cell_size),
            graphics::WHITE
        )?;

        let cell_mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::stroke(1.0),
            Rect::new(self.pos.x, self.pos.y, self.cell_size, self.cell_size),
            (243, 243, 237, 50).into()
        )?;

        let grid_mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::stroke(1.0),
            Rect::new(self.pos.x, self.pos.y, self.cell_size * BOARD_WIDTH as f32, self.cell_size * BOARD_HEIGHT as f32),
            (243, 243, 237, 50).into()
        )?;

        graphics::draw(ctx, &grid_mesh, ([0.0, 0.0],))?;

        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                let pos = [x as f32 * self.cell_size, y as f32 * self.cell_size];

                let block = self.get(x, y).unwrap();
                if block.is_some() {
                    let color = PIECE_COLORS[&block.unwrap()];
                    graphics::draw(ctx, &block_mesh, (pos, color))?;
                }

                graphics::draw(ctx, &cell_mesh, (pos,))?;
            }
        }

        Ok(())
    }
}
use ggez::{mint, graphics, Context, GameResult};
use ggez::graphics::{DrawMode, Mesh, Rect};
use anyhow::{Result, anyhow};
use crate::piece::{PIECE_COLORS, PIECE_SHAPES, Shape};

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
        if x >= BOARD_WIDTH || y >= BOARD_HEIGHT {
            Err(anyhow!("out of bounds index"))
        } else {
            Ok(y * BOARD_WIDTH + x)
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

    pub fn put(&mut self, mut x: usize, y: usize, piece: Shape) -> Result<()> {
        if y < 16 {
            return Err(anyhow!("invalid y position"));
        }

        let shape = &PIECE_SHAPES[&piece];
        let shape_height = shape.len();
        let shape_width = shape[0].len();

        if x > BOARD_WIDTH - shape_width {
            x = BOARD_WIDTH - shape_width;
        }

        let mut drop_y = 0;
        'drop: while drop_y <= BOARD_HEIGHT - shape_height {
            for piece_y in 0..shape_height {
                let row = &shape[piece_y];
                for piece_x in 0..shape_width {
                    if row[piece_x] == 1 {
                        if self.get(x + piece_x, drop_y + piece_y).unwrap().is_some() {
                            break 'drop;
                        }
                    }
                }
            }
            drop_y += 1;
        }
        drop_y -= 1;

        for piece_y in 0..shape_height {
            let row = &shape[piece_y];
            for piece_x in 0..shape_width {
                if row[piece_x] == 1 {
                    self.set(x + piece_x, drop_y + piece_y, piece).unwrap();
                }
            }
        }

        Ok(())
    }

    pub fn click(&self, x: f32, y: f32) -> Option<(usize, usize)> {
        if x > self.pos.x &&
           y > self.pos.y &&
           x < (self.pos.x + BOARD_WIDTH as f32 * self.cell_size) &&
           y < (self.pos.y + BOARD_HEIGHT as f32 * self.cell_size) {
            let grid_x = (x - self.pos.x) as usize / self.cell_size as usize;
            let grid_y = (y - self.pos.y) as usize / self.cell_size as usize;

            Some((grid_x, grid_y))
        } else {
            None
        }
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
            (243, 243, 237, 25).into()
        )?;

        let grid_mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::stroke(1.0),
            Rect::new(self.pos.x, self.pos.y, self.cell_size * BOARD_WIDTH as f32, self.cell_size * BOARD_HEIGHT as f32),
            (243, 243, 237, 25).into()
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
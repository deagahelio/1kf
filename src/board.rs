use ggez::{mint, graphics, Context, GameResult};
use ggez::graphics::{DrawMode, Mesh, Rect};
use anyhow::{Result, anyhow};
use crate::piece::{PIECE_COLORS, PIECE_SHAPES, Rotation, Shape, rotate_piece};

pub struct Board {
    pos: mint::Point2<f32>,
    cell_size: f32,
    width: usize,
    height: usize,
    grid: Vec<Option<Shape>>,
}

impl Board {
    pub fn new(pos: mint::Point2<f32>, cell_size: f32, width: usize, height: usize) -> Self {
        Self {
            pos,
            cell_size,
            width,
            height,
            grid: vec![None; width * height],
        }
    }

    pub fn index(&self, x: usize, y: usize) -> Result<usize> {
        if x >= self.width || y >= self.height {
            Err(anyhow!("out of bounds index"))
        } else {
            Ok(y * self.width + x)
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Result<Option<Shape>> {
        Ok(self.grid[self.index(x, y)?])
    }

    pub fn set(&mut self, x: usize, y: usize, new: Option<Shape>) -> Result<Option<Shape>> {
        let index = self.index(x, y)?;
        let old = self.grid[index];
        self.grid[index] = new;
        Ok(old)
    }

    pub fn put(&mut self, mut x: usize, rotation: Rotation, piece: Shape) -> Result<()> {
        let shape = rotate_piece(&PIECE_SHAPES[&piece], rotation);
        let shape_height = shape.len();
        let shape_width = shape[0].len();

        if x > self.width - shape_width {
            x = self.width - shape_width;
        }

        let mut y = 0;
        'drop: while y <= self.height - shape_height {
            for piece_y in 0..shape_height {
                let row = &shape[piece_y];
                for piece_x in 0..shape_width {
                    if row[piece_x] == 1 {
                        if self.get(x + piece_x, y + piece_y).unwrap().is_some() {
                            break 'drop;
                        }
                    }
                }
            }
            y += 1;
        }
        y -= 1;

        for piece_y in 0..shape_height {
            let row = &shape[piece_y];
            for piece_x in 0..shape_width {
                if row[piece_x] == 1 {
                    self.set(x + piece_x, y + piece_y, Some(piece)).unwrap();
                }
            }
        }

        Ok(())
    }

    pub fn get_rotation(&self, grid_y: usize) -> Option<Rotation> {
        match self.height - grid_y {
            4 => Some(Rotation::OneEighty),
            3 => Some(Rotation::Clockwise),
            2 => Some(Rotation::Normal),
            1 => Some(Rotation::CounterClockwise),
            _ => None,
        }
    }

    pub fn clear_line(&mut self, line_y: usize) {
        for y in (0..line_y).rev() {
            for x in 0..self.width {
                self.set(x, y + 1, self.get(x, y).unwrap()).unwrap();
            }
        }
    }

    pub fn clear_lines(&mut self) {
        'lines: for y in 0..self.height {
            for x in 0..self.width {
                if self.get(x, y).unwrap().is_none() {
                    continue 'lines;
                }
            }
            self.clear_line(y);
        }
    }

    pub fn click(&self, x: f32, y: f32) -> Option<(usize, usize)> {
        if x > self.pos.x &&
           y > self.pos.y &&
           x < (self.pos.x + self.width as f32 * self.cell_size) &&
           y < (self.pos.y + self.height as f32 * self.cell_size) {
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
            Rect::new(self.pos.x, self.pos.y, self.cell_size * self.width as f32, self.cell_size * self.height as f32),
            (243, 243, 237, 25).into()
        )?;

        let highlight_mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(self.pos.x, self.pos.y + (self.height - 4) as f32 * self.cell_size, self.cell_size * self.width as f32, self.cell_size * 4.0),
            (255, 0, 0, 25).into()
        )?;

        let highlight_outline_mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::stroke(3.0),
            Rect::new(self.pos.x, self.pos.y + (self.height - 4) as f32 * self.cell_size, self.cell_size * self.width as f32, self.cell_size * 4.0),
            (255, 0, 0, 255).into()
        )?;

        graphics::draw(ctx, &grid_mesh, ([0.0, 0.0],))?;

        for y in 0..self.height {
            for x in 0..self.width{
                let pos = [x as f32 * self.cell_size, y as f32 * self.cell_size];

                let block = self.get(x, y).unwrap();
                if block.is_some() {
                    let color = PIECE_COLORS[&block.unwrap()];
                    graphics::draw(ctx, &block_mesh, (pos, color))?;
                }

                graphics::draw(ctx, &cell_mesh, (pos,))?;
            }
        }

        graphics::draw(ctx, &highlight_mesh, ([0.0, 0.0],))?;
        graphics::draw(ctx, &highlight_outline_mesh, ([0.0, 0.0],))?;

        Ok(())
    }
}
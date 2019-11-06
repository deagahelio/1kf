use ggez::{mint, graphics, Context, GameResult};
use ggez::graphics::{DrawMode, Mesh, Rect};
use ggez::input::mouse;
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

    pub fn put(&mut self, x: usize, rotation: Rotation, piece: Shape) -> Result<()> {
        let shape = rotate_piece(&PIECE_SHAPES[&piece], rotation);

        let (x, y) = self.drop(x, &shape);

        for (piece_y, row) in shape.iter().enumerate() {
            for (piece_x, col) in row.iter().enumerate() {
                if *col == 1 {
                    self.set(x + piece_x, y + piece_y, Some(piece)).unwrap();
                }
            }
        }

        Ok(())
    }

    pub fn drop(&self, mut x: usize, shape: &Vec<Vec<u8>>) -> (usize, usize) {
        if x > self.width - shape[0].len() {
            x = self.width - shape[0].len();
        }

        let mut y = 0;
        'drop: while y <= self.height - shape.len() {
            for (piece_y, row) in shape.iter().enumerate() {
                for (piece_x, col) in row.iter().enumerate() {
                    if *col == 1 {
                        if self.get(x + piece_x, y + piece_y).unwrap().is_some() {
                            break 'drop;
                        }
                    }
                }
            }
            y += 1;
        }
        (x, y - 1)
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

    pub fn draw_ghost(&self, ctx: &mut Context, piece: &Shape) -> GameResult {
        let mint::Point2 { x, y } = mouse::position(ctx);

        if let Some((grid_x, grid_y)) = self.click(x, y) {
            if let Some(rotation) = self.get_rotation(grid_y) {
                let shape = rotate_piece(&PIECE_SHAPES[piece], rotation);
                let (grid_x, grid_y) = self.drop(grid_x, &shape);

                let block_mesh = Mesh::new_rectangle(
                    ctx,
                    DrawMode::fill(),
                    Rect::new(self.pos.x + grid_x as f32 * self.cell_size,
                              self.pos.y + grid_y as f32 * self.cell_size,
                              self.cell_size,
                              self.cell_size),
                    (255, 255, 255, 25).into()
                )?;

                for (piece_y, row) in shape.iter().enumerate() {
                    for (piece_x, col) in row.iter().enumerate() {
                        if *col == 1 {
                            let pos = [piece_x as f32 * self.cell_size, piece_y as f32 * self.cell_size];
                            graphics::draw(ctx, &block_mesh, (pos,))?;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub fn draw(&self, ctx: &mut Context, current: &Shape) -> GameResult {
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
            for x in 0..self.width {
                let pos = [x as f32 * self.cell_size, y as f32 * self.cell_size];

                let block = self.get(x, y).unwrap();
                if block.is_some() {
                    let color = PIECE_COLORS[&block.unwrap()];
                    graphics::draw(ctx, &block_mesh, (pos, color))?;
                }

                graphics::draw(ctx, &cell_mesh, (pos,))?;
            }
        }

        self.draw_ghost(ctx, current)?;

        graphics::draw(ctx, &highlight_mesh, ([0.0, 0.0],))?;
        graphics::draw(ctx, &highlight_outline_mesh, ([0.0, 0.0],))?;

        Ok(())
    }
}
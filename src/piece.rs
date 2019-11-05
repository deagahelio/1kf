use ggez::graphics::Color;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref PIECE_COLORS: HashMap<Shape, Color> = {
        [(Shape::I, Color::from_rgb(65,  175, 222)),
         (Shape::J, Color::from_rgb(25,  131, 191)),
         (Shape::L, Color::from_rgb(239, 149,  53)),
         (Shape::O, Color::from_rgb(247, 211,  62)),
         (Shape::S, Color::from_rgb(102, 198,  92)),
         (Shape::T, Color::from_rgb(180,  81, 172)),
         (Shape::Z, Color::from_rgb(239,  98,  77)),].iter().cloned().collect()
    };
    pub static ref PIECE_SHAPES: HashMap<Shape, Vec<Vec<u8>>> = {
        [(Shape::I, vec![vec![1, 1, 1, 1]]),
         (Shape::J, vec![vec![1, 0, 0],
                         vec![1, 1, 1]]),
         (Shape::L, vec![vec![0, 0, 1],
                         vec![1, 1, 1]]),
         (Shape::O, vec![vec![1, 1],
                         vec![1, 1]]),
         (Shape::S, vec![vec![0, 1, 1],
                         vec![1, 1, 0]]),
         (Shape::T, vec![vec![0, 1, 0],
                         vec![1, 1, 1]]),
         (Shape::Z, vec![vec![1, 1, 0],
                         vec![0, 1, 1]])].iter().cloned().collect()
    };
}

pub fn rotate_piece(piece: &Vec<Vec<u8>>, rotation: Rotation) -> Vec<Vec<u8>> {
    match rotation {
        Rotation::OneEighty => {
            let mut new = piece.clone();
            for row in &mut new {
                row.reverse();
            }
            new.reverse();
            new
        },
        Rotation::Clockwise | Rotation::CounterClockwise => {
            let mut new = Vec::with_capacity(piece[0].len());
            for y in 0..piece[0].len() {
                let mut new_row = Vec::with_capacity(piece.len());
                for x in 0..piece.len() {
                    if rotation == Rotation::Clockwise {
                        new_row.push(piece[piece.len() - x - 1][y]);
                    } else {
                        new_row.push(piece[x][piece[0].len() - y - 1]);
                    }
                }
                new.push(new_row);
            }
            new
        },
        Rotation::Normal => piece.clone(),
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Rotation {
    OneEighty,
    Clockwise,
    Normal,
    CounterClockwise,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Shape {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}
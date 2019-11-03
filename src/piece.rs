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
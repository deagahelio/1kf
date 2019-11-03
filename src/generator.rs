use rand::seq::SliceRandom;
use crate::piece::Shape;

pub trait PieceGenerator {
    fn peek_next(&self, amount: usize) -> Vec<Shape>;
    fn get_next(&mut self) -> Shape;
}

pub struct SevenBagGenerator {
    current_bag: Vec<Shape>,
    next_bag: Vec<Shape>,
}

impl SevenBagGenerator {
    pub fn new() -> Self {
        Self {
            current_bag: SevenBagGenerator::generate_bag(),
            next_bag: SevenBagGenerator::generate_bag(),
        }
    }

    pub fn generate_bag() -> Vec<Shape> {
        let mut bag = [
            Shape::I,
            Shape::J,
            Shape::L,
            Shape::O,
            Shape::S,
            Shape::T,
            Shape::Z,
        ];
        &mut bag.shuffle(&mut rand::thread_rng());
        bag.to_vec()
    }
}

impl PieceGenerator for SevenBagGenerator {
    fn peek_next(&self, amount: usize) -> Vec<Shape> {
        let mut bags = Vec::with_capacity(14);
        bags.append(&mut self.current_bag.clone());
        bags.append(&mut self.next_bag.clone());

        bags[..amount].to_vec()
    }

    fn get_next(&mut self) -> Shape {
        if self.current_bag.is_empty() {
            let new_bag = SevenBagGenerator::generate_bag();
            self.current_bag = std::mem::replace(&mut self.next_bag, new_bag);
        }

        self.current_bag.remove(0)
    }
}
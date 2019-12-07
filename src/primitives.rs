use std::fmt::{Debug, Formatter, Result};

#[derive(Clone)]
pub struct Vertex(pub [f32; 3]);

impl Vertex {
    pub fn dot(&self, other: &Vertex) -> f32 {
        let Vertex([x1, y1, z1]) = self;
        let Vertex([x2, y2, z2]) = other;
        return x1 * x2 + y1 * y2 + z1 * z2;
    }
}

impl Debug for Vertex {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let Vertex([x, y, z]) = self;
        write!(f, "({}, {}, {})", x, y, z)
    }
}

#[derive(Clone, Debug)]
pub enum Face {
    Triangle([usize; 3]),
    Quad([usize; 4]), 
}


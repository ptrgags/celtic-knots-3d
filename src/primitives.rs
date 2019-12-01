use std::fmt::{Debug, Formatter, Result};

#[derive(Clone)]
pub struct Vertex(pub [f32; 3]);

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

